use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

fn main() {
    nannou::app(model).run();
}

struct Player {
    position: Point2,
}

struct CamelWarrior {
    position: Point2,
}

#[derive(Default, Clone)]
struct Tile {
    t_wall: bool,
    r_wall: bool,
    l_wall: bool,
    b_wall: bool,
}

impl Tile {
    fn randomize_all_walls(&mut self) {
        let mut rng = StdRng::from_entropy();
        // flip 4 coins
        self.t_wall = rng.gen_bool(0.5);
        self.b_wall = rng.gen_bool(0.5);
        self.l_wall = rng.gen_bool(0.5);
        self.t_wall = rng.gen_bool(0.5);

        // check if they're all walls; make one of them a corridor
        if self.b_wall && self.t_wall && self.l_wall && self.r_wall {
            self.t_wall = false; // randomize this ?
        }
    }
}

// Plan: "field of view" is 3x3 grid around the player

struct Model {
    grid: Vec<Vec<Tile>>,
    player: Player,
    camel_warrior: CamelWarrior,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1200, 900)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut model = Model {
        grid: vec![vec![Tile::default(); 20]; 20],
        player: Player {
            position: (10., 10.).into(),
        },
        camel_warrior: CamelWarrior {
            position: (15., 15.).into(),
        },
    };

    // generate walls for the first tile
    let (i, j) = model.player.position.into();
    model.grid[i as usize][j as usize].randomize_all_walls();

    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let dir: Point2 = match key {
        Key::Up => (0., -1.),
        Key::Down => (0., 1.),
        Key::Left => (-1., 0.),
        Key::Right => (1., 0.),
        _ => return,
    }
    .into();

    // move the player in `dir`
    model.player.position += dir;

    // generate walls for the current tile
    let (i, j) = model.player.position.into();
    model.grid[i as usize][j as usize].randomize_all_walls();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    let n = model.grid.len();
    let tile_w = win.w() / n as f32;
    let tile_h = win.h() / n as f32;

    for x in 0..n {
        for y in 0..n {
            debug_assert_eq!(model.grid[x].len(), n);

            // draw the current tile
            let pos_x = (win.left() + (tile_w / 2.0)) + tile_w * x as f32;
            let pos_y = (win.top() - (tile_h / 2.0)) - tile_h * y as f32;

            // for each wall, draw lines
            let tile = &model.grid[x][y];
            let mut point_pairs = vec![];
            if tile.t_wall {
                point_pairs.push(((pos_x, pos_y), (pos_x + tile_w, pos_y)));
            }
            if tile.b_wall {
                point_pairs.push(((pos_x, pos_y + tile_h), (pos_x + tile_w, pos_y + tile_h)));
            }
            if tile.l_wall {
                point_pairs.push(((pos_x, pos_y), (pos_x, pos_y + tile_h)));
            }
            if tile.r_wall {
                point_pairs.push(((pos_x + tile_w, pos_y), (pos_x + tile_w, pos_y + tile_h)));
            }

            for ((sx, sy), (ex, ey)) in point_pairs {
                draw.line()
                    .start((sx, sy).into())
                    .end((ex, ey).into())
                    .weight(1.);
            }
        }
    }

    // draw the player
    let (x, y) = model.player.position.into();
    let pos_x = (win.left() + (tile_w / 2.0)) + tile_w * x as f32;
    let pos_y = (win.top() - (tile_h / 2.0)) - tile_h * y as f32;

    draw.ellipse()
        .x_y(pos_x + tile_w / 2., pos_y + tile_h / 2.)
        .radius(10.0)
        .no_fill()
        .stroke(rgba(1., 0., 0., 1.))
        .stroke_weight(1.);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {}
