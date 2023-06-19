use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const NUM_OF_ROWS: usize = 21;
const NUM_OF_COLS: usize = 21;

fn main() {
    nannou::app(init).run();
}

#[derive(Default, Clone, Debug, Copy)]
struct GridCoordinates {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Neighbor {
    cell: Option<Cell>,
    dir: Direction,
}

struct Player {
    position: Point2,
    grid_coordinates: GridCoordinates,
}

struct CamelWarrior {
    position: Point2,
    grid_coordinates: GridCoordinates,
}

// Stack
// Dict for finalized cells

#[derive(Default, Clone, Debug, Copy)]
struct Cell {
    t_wall: bool,
    r_wall: bool,
    l_wall: bool,
    b_wall: bool,
    in_maze: bool,
    finalized: bool,
    grid_coordinates: GridCoordinates,
}

impl Cell {
    fn connect_to_neighbors(&mut self, neighbors: Vec<Neighbor>) {
        neighbors
            .into_iter()
            .for_each(|neighbor| match neighbor.dir {
                Direction::Up => self.t_wall = false,
                Direction::Down => self.b_wall = false,
                Direction::Left => self.l_wall = false,
                Direction::Right => self.r_wall = false,
            });
        self.in_maze = true;
    }
}

struct Model {
    grid: Vec<Vec<Cell>>,
    player: Player,
    camel_warrior: CamelWarrior,
}

fn init(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1200, 900)
        .view(view)
        // .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut model = Model {
        grid: vec![vec![Cell::default(); NUM_OF_ROWS]; NUM_OF_COLS],
        player: Player {
            position: (0.0, 0.0).into(),
            grid_coordinates: GridCoordinates {
                x: NUM_OF_COLS / 2,
                y: NUM_OF_ROWS / 2,
            },
        },
        camel_warrior: CamelWarrior {
            position: (15.0, 15.0).into(),
            grid_coordinates: GridCoordinates { x: 15, y: 15 },
        },
    };

    // Set grid coordinates for every cell
    for y in 0..NUM_OF_COLS {
        for x in 0..NUM_OF_ROWS {
            model.grid[x][y].grid_coordinates.x = x;
            model.grid[x][y].grid_coordinates.y = y;
            model.grid[x][y].t_wall = false;
            model.grid[x][y].b_wall = false;
            model.grid[x][y].l_wall = false;
            model.grid[x][y].r_wall = false;
            model.grid[x][y].in_maze = false;
            model.grid[x][y].finalized = false;
        }
    }

    let neighbors = get_neighbors(
        model.grid.clone(),
        model.grid[model.player.grid_coordinates.x as usize]
            [model.player.grid_coordinates.y as usize],
    );
    let connection_directions = directions_to_connect_to(neighbors);

    // generate walls for the first cell
    let GridCoordinates { x, y } = model.player.grid_coordinates;
    model.grid[x as usize][y as usize].connect_to_neighbors(connection_directions);
    model.grid[x as usize][y as usize].in_maze = true;
    model.grid[x as usize][y as usize].finalized = true;

    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let dir: Point2 = match key {
        Key::Up => (0.0, 1.0),
        Key::Down => (0.0, -1.0),
        Key::Left => (-1.0, 0.0),
        Key::Right => (1.0, 0.0),
        _ => return,
    }
    .into();

    // move the player in the Nannou coordinate system and in our maze's grid
    // TODO: Account for being on the edge of the maze grid, we should't let the player move out of the maze boundaries.
    model.player.position += dir;
    model.player.grid_coordinates.x =
        (model.player.grid_coordinates.x as isize + dir[0] as isize) as usize;
    model.player.grid_coordinates.y =
        (model.player.grid_coordinates.y as isize + dir[1] as isize) as usize;

    let neighbors = get_neighbors(
        model.grid.clone(),
        model.grid[model.player.grid_coordinates.x as usize]
            [model.player.grid_coordinates.y as usize],
    );
    let neighbors_to_connect_to = directions_to_connect_to(neighbors);

    model.player.position;
    // generate walls for the current cell
    let GridCoordinates { x, y } = model.player.grid_coordinates;
    model.grid[x as usize][y as usize].connect_to_neighbors(neighbors_to_connect_to);
    model.grid[x as usize][y as usize].finalized = true;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();

    let cell_w = win.w() / NUM_OF_COLS as f32;
    let cell_h = win.h() / NUM_OF_ROWS as f32;

    let draw = app.draw();

    draw.background().color(WHITE);

    // Draw the walls for each cell
    for x in 0..NUM_OF_COLS {
        for y in 0..NUM_OF_ROWS {
            // for each wall, draw lines
            let cell = model.grid[x][y];

            let left_boundary_of_cell = cell.grid_coordinates.x as f32 * cell_w;
            let top_boundary_of_cell = cell.grid_coordinates.y as f32 * cell_h;

            if cell.in_maze {
                let mut point_pairs = vec![];
                if cell.t_wall {
                    point_pairs.push((
                        (left_boundary_of_cell, top_boundary_of_cell),
                        (left_boundary_of_cell + cell_w, top_boundary_of_cell),
                    ));
                }
                if cell.b_wall {
                    point_pairs.push((
                        (left_boundary_of_cell, top_boundary_of_cell + cell_h),
                        (
                            left_boundary_of_cell + cell_w,
                            top_boundary_of_cell + cell_h,
                        ),
                    ));
                }
                if cell.l_wall {
                    point_pairs.push((
                        (left_boundary_of_cell, top_boundary_of_cell),
                        (left_boundary_of_cell, top_boundary_of_cell + cell_h),
                    ));
                }
                if cell.r_wall {
                    point_pairs.push((
                        (left_boundary_of_cell + cell_w, top_boundary_of_cell),
                        (
                            left_boundary_of_cell + cell_w,
                            top_boundary_of_cell + cell_h,
                        ),
                    ));
                }

                for ((sx, sy), (ex, ey)) in point_pairs {
                    draw.line()
                        .start((sx, sy).into())
                        .end((ex, ey).into())
                        .weight(1.0);
                }
            }
        }
    }

    // draw the player
    let (x, y) = model.player.position.into();

    draw.ellipse()
        .x_y(x as f32 * cell_w, y as f32 * cell_h)
        .radius(10.0)
        .no_fill()
        .stroke(rgba(1.0, 0.0, 0.0, 1.0))
        .stroke_weight(1.0);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

// fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {}

fn get_neighbor(grid: Vec<Vec<Cell>>, cell: Cell, dir: Direction) -> Cell {
    match dir {
        Direction::Up => {
            grid[cell.grid_coordinates.x as usize][(cell.grid_coordinates.y + 1) as usize]
        }
        Direction::Down => {
            grid[cell.grid_coordinates.x as usize][(cell.grid_coordinates.y - 1) as usize]
        }
        Direction::Left => {
            grid[(cell.grid_coordinates.x - 1) as usize][cell.grid_coordinates.y as usize]
        }
        Direction::Right => {
            grid[(cell.grid_coordinates.x + 1) as usize][cell.grid_coordinates.y as usize]
        }
    }
}

fn get_neighbors(grid: Vec<Vec<Cell>>, cell: Cell) -> Vec<Neighbor> {
    let mut neighbors = vec![];

    if cell.grid_coordinates.y + 1 < NUM_OF_ROWS {
        neighbors.push(Neighbor {
            cell: Some(
                grid[cell.grid_coordinates.x as usize][(cell.grid_coordinates.y + 1) as usize],
            ),
            dir: Direction::Up,
        })
    };

    if cell.grid_coordinates.y > 0 {
        neighbors.push(Neighbor {
            cell: Some(
                grid[cell.grid_coordinates.x as usize][(cell.grid_coordinates.y - 1) as usize],
            ),
            dir: Direction::Down,
        })
    };

    if cell.grid_coordinates.x > 0 {
        neighbors.push(Neighbor {
            cell: Some(
                grid[(cell.grid_coordinates.x - 1) as usize][cell.grid_coordinates.y as usize],
            ),
            dir: Direction::Left,
        })
    };

    if cell.grid_coordinates.x + 1 < NUM_OF_COLS {
        neighbors.push(Neighbor {
            cell: Some(
                grid[(cell.grid_coordinates.x + 1) as usize][cell.grid_coordinates.y as usize],
            ),
            dir: Direction::Right,
        })
    };

    return neighbors;
}

fn directions_to_connect_to(mut neighbors: Vec<Neighbor>) -> Vec<Neighbor> {
    let mut rng = StdRng::from_entropy();
    let mut neighbors_to_connect_to: Vec<Neighbor> = vec![];

    // Randomly choose which neighbors to include
    for _i in 0..neighbors.len() {
        let neighbor = neighbors.pop().unwrap();

        // Check if the neighbor already has a wall there, if they do then don't connect to it.
        if match neighbor.dir {
            Direction::Up => !neighbor.cell.unwrap().b_wall,
            Direction::Down => !neighbor.cell.unwrap().t_wall,
            Direction::Left => !neighbor.cell.unwrap().r_wall,
            Direction::Right => !neighbor.cell.unwrap().l_wall,
        } && rng.gen_bool(0.5)
        {
            neighbors_to_connect_to.push(neighbor);
        } else {
            neighbors.insert(0, neighbor);
        }
    }

    // TODO: Account for edges where there are no neighbors

    if neighbors_to_connect_to.len() == 0 {
        neighbors_to_connect_to.push(neighbors[0]);
    }

    return neighbors_to_connect_to;
}
