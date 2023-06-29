use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const NUM_OF_ROWS: usize = 20;
const NUM_OF_COLS: usize = 20;

fn main() {
    nannou::app(init).run();
}

#[derive(Default, Clone, Debug, Copy, PartialEq)]
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

#[derive(Debug)]
struct Neighbor {
    cell: Option<Cell>,
    dir: Direction,
}

#[derive(Debug)]
struct Player {
    grid_coordinates: GridCoordinates,
}

struct CamelWarrior {
    grid_coordinates: GridCoordinates,
}

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

struct Model {
    grid: Grid,
    player: Player,
    camel_warrior: CamelWarrior,
    exit: GridCoordinates,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn connect_neighbors(&mut self, cell_coords: GridCoordinates, directions: Vec<Direction>) {
        directions.into_iter().for_each(|direction| {
            let cell = &mut self.cells[cell_coords.x][cell_coords.y];
            let mut neighbor: &mut Cell;

            match direction {
                Direction::Up => {
                    cell.t_wall = false;

                    // Remove the bottom wall of the cell above
                    neighbor =
                        &mut self.cells[cell_coords.x as usize][(cell_coords.y + 1) as usize];
                    neighbor.b_wall = false;
                }
                Direction::Down => {
                    cell.b_wall = false;

                    // Remove the top wall of the cell below
                    neighbor =
                        &mut self.cells[cell_coords.x as usize][(cell_coords.y - 1) as usize];
                    neighbor.t_wall = false;
                }
                Direction::Left => {
                    cell.l_wall = false;

                    // Remove the right wall of the cell to the left
                    neighbor =
                        &mut self.cells[(cell_coords.x - 1) as usize][cell_coords.y as usize];
                    neighbor.r_wall = false;
                }
                Direction::Right => {
                    cell.r_wall = false;

                    // Remove the left wall of the cell to the right
                    neighbor =
                        &mut self.cells[(cell_coords.x + 1) as usize][cell_coords.y as usize];
                    neighbor.l_wall = false;
                }
            }
            neighbor.in_maze = true;
        });

        let cell = &mut self.cells[cell_coords.x][cell_coords.y];
        cell.in_maze = true;
        cell.finalized = true;
    }

    pub fn check_for_wall(&mut self, cell_coords: GridCoordinates, direction: Key) -> bool {
        match direction {
            Key::Up => self.cells[cell_coords.x][cell_coords.y].t_wall,
            Key::Down => self.cells[cell_coords.x][cell_coords.y].b_wall,
            Key::Left => self.cells[cell_coords.x][cell_coords.y].l_wall,
            Key::Right => self.cells[cell_coords.x][cell_coords.y].r_wall,
            _ => false,
        }
    }
}

fn init(app: &App) -> Model {
    assert_eq!(NUM_OF_COLS, NUM_OF_COLS);

    let _window = app
        .new_window()
        .size(900, 900)
        .view(view)
        // .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut rng = StdRng::from_entropy();

    // Randomly select the starting coordinates for the player
    let player_coords = GridCoordinates {
        x: rng.gen_range(0..NUM_OF_COLS),
        y: rng.gen_range(0..NUM_OF_ROWS),
    };
    let cammel_warrior_coords = GridCoordinates {
        x: rng.gen_range(0..NUM_OF_COLS),
        y: rng.gen_range(0..NUM_OF_ROWS),
    };

    let mut model = Model {
        grid: Grid {
            cells: vec![vec![Cell::default(); NUM_OF_ROWS]; NUM_OF_COLS],
        },
        player: Player {
            grid_coordinates: player_coords,
        },
        camel_warrior: CamelWarrior {
            grid_coordinates: cammel_warrior_coords,
        },
        exit: GridCoordinates {
            x: rng.gen_range(0..NUM_OF_COLS),
            y: rng.gen_range(0..NUM_OF_ROWS),
        },
    };

    // Initialize every cell
    for x in 0..NUM_OF_COLS {
        for y in 0..NUM_OF_ROWS {
            model.grid.cells[x][y].grid_coordinates.x = x;
            model.grid.cells[x][y].grid_coordinates.y = y;
            model.grid.cells[x][y].t_wall = true;
            model.grid.cells[x][y].b_wall = true;
            model.grid.cells[x][y].l_wall = true;
            model.grid.cells[x][y].r_wall = true;
            model.grid.cells[x][y].in_maze = false;
            model.grid.cells[x][y].finalized = false;
        }
    }

    // Get the starting cell
    let starting_cell = model.grid.cells[model.player.grid_coordinates.x as usize]
        [model.player.grid_coordinates.y as usize];

    // Get the starting cell's neighbors and probabilstically connect it to them
    let neighbors = get_neighbors(model.grid.cells.clone(), starting_cell);
    let connection_directions = directions_to_connect_to(neighbors);

    // Remove the walls in the direction of the connected neighbors
    model
        .grid
        .connect_neighbors(model.player.grid_coordinates, connection_directions);

    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    // TODO: Add message and steps for when you are near the camel warrior.

    // Move the player but only if there is not a wall in the direction its trying to move
    if !model
        .grid
        .check_for_wall(model.player.grid_coordinates, key.into())
    {
        let movement_vec: (isize, isize) = match key {
            Key::Up => (0, 1),
            Key::Down => (0, -1),
            Key::Left => (-1, 0),
            Key::Right => (1, 0),
            _ => return,
        }
        .into();

        model.player.grid_coordinates.x =
            (model.player.grid_coordinates.x as isize + movement_vec.0) as usize;
        model.player.grid_coordinates.y =
            (model.player.grid_coordinates.y as isize + movement_vec.1) as usize;

        // Check if the player is now at the exit
        if model.player.grid_coordinates == model.exit {
            println!("🎉 YOU WON! 🎉");
            app.quit();
        }

        let grid_copy = model.grid.cells.clone();

        let cell = model.grid.cells[model.player.grid_coordinates.x as usize]
            [model.player.grid_coordinates.y as usize];

        if !cell.finalized {
            let neighbors = get_neighbors(grid_copy, cell.clone());
            let connection_directions = directions_to_connect_to(neighbors);

            // generate walls for the first cell
            model
                .grid
                .connect_neighbors(model.player.grid_coordinates, connection_directions);
        }
    } else {
        // TODO: Play error sound.
        println!("Can not move {:?}", key);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(WHITE);

    // Difference between Maze's grid origin [0;0] and Nannou's canvas origin (0,0) in pixels
    let x_offset = -win.w() / 2.0;
    let y_offset = -win.h() / 2.0;

    let cell_w = win.w() / NUM_OF_COLS as f32;
    let cell_h = win.h() / NUM_OF_ROWS as f32;

    let draw_walls = |color: Srgb<u8>, cell_filter: &dyn Fn(Cell) -> bool| -> () {
        for column in 0..NUM_OF_COLS {
            for row in 0..NUM_OF_ROWS {
                // for each wall, draw lines
                let cell = model.grid.cells[column][row];

                let left_boundary_of_cell = cell.grid_coordinates.x as f32 * cell_w + x_offset;
                let bottom_boundary_of_cell = cell.grid_coordinates.y as f32 * cell_h + y_offset;

                if cell_filter(cell) {
                    let mut point_pairs = vec![];
                    if cell.t_wall {
                        point_pairs.push((
                            (left_boundary_of_cell, bottom_boundary_of_cell + cell_h),
                            (
                                left_boundary_of_cell + cell_w,
                                bottom_boundary_of_cell + cell_h,
                            ),
                        ));
                    }
                    if cell.b_wall {
                        point_pairs.push((
                            (left_boundary_of_cell, bottom_boundary_of_cell),
                            (left_boundary_of_cell + cell_w, bottom_boundary_of_cell),
                        ));
                    }
                    if cell.l_wall {
                        point_pairs.push((
                            (left_boundary_of_cell, bottom_boundary_of_cell),
                            (left_boundary_of_cell, bottom_boundary_of_cell + cell_h),
                        ));
                    }
                    if cell.r_wall {
                        point_pairs.push((
                            (left_boundary_of_cell + cell_w, bottom_boundary_of_cell),
                            (
                                left_boundary_of_cell + cell_w,
                                bottom_boundary_of_cell + cell_h,
                            ),
                        ));
                    }

                    for ((sx, sy), (ex, ey)) in point_pairs {
                        draw.line()
                            .color(color)
                            .start((sx, sy).into())
                            .end((ex, ey).into())
                            .weight(1.0);
                    }
                }
            }
        }
    };

    // Draw the walls for every non-finalized cell in the maze, for debugging purposes only
    // draw_walls(YELLOWGREEN, &|cell| !cell.finalized && cell.in_maze);

    // Draw the maze walls
    draw_walls(BLACK, &|cell| cell.finalized);

    // START - Draw entities in the maze
    let grid_coords_to_nannou_position = |coords: GridCoordinates| -> (f32, f32) {
        let GridCoordinates { x, y } = coords;
        (
            (x as f32 + 0.5) * cell_w + x_offset,
            (y as f32 + 0.5) * cell_h + y_offset,
        )
    };

    // Draw the player
    let player_position = grid_coords_to_nannou_position(model.player.grid_coordinates);
    let player_radius = cell_w * 0.4 / 2.0;
    draw.ellipse()
        .x_y(player_position.0, player_position.1)
        .radius(player_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(1.0);

    // Draw the exit if the player is 2 or less moves away from it
    // TODO: Change this to line of sight?
    if distance_between_coords(model.player.grid_coordinates, model.exit) <= 2 {
        let exit_position = grid_coords_to_nannou_position(model.exit);
        draw.ellipse()
            .x_y(exit_position.0, exit_position.1)
            .radius(player_radius)
            .color(GREEN)
            .stroke(GREEN)
            .stroke_weight(1.0);
    }
    // END - Draw entities in the maze

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

// fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {}

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

fn directions_to_connect_to(mut neighbors: Vec<Neighbor>) -> Vec<Direction> {
    let mut rng = StdRng::from_entropy();
    let mut directions_to_connect_to: Vec<Direction> = vec![];

    // Randomly choose which neighbors to include
    for _i in 0..neighbors.len() {
        let neighbor = neighbors.pop().unwrap();

        // Check if the neighbor already has a wall there, if they do then don't connect to it.
        if !neighbor.cell.unwrap().finalized {
            if rng.gen_bool(0.5) {
                directions_to_connect_to.push(neighbor.dir);
            } else {
                neighbors.insert(0, neighbor);
            }
        }
    }

    // Ensure that there is at least always one direction to connect to when possible
    if directions_to_connect_to.len() == 0 && neighbors.len() != 0 {
        directions_to_connect_to.push(neighbors[0].dir);
    }

    return directions_to_connect_to;
}

fn distance_between_coords(coords_a: GridCoordinates, coords_b: GridCoordinates) -> usize {
    let mut distance = 0;

    distance += coords_a.x.abs_diff(coords_b.x);
    distance += coords_a.y.abs_diff(coords_b.y);

    distance
}
