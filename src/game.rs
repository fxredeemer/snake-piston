use piston::input::keyboard::Key;
use piston_window::G2d;
use piston_window::*;
use piston_window::Glyphs;

use crate::state::*;
use crate::snake::*;
use crate::point::*;
use crate::food::*;
use crate::consts::*;
use crate::level::*;

pub struct Game {
    pub snake: Snake,
    pub walls: Vec<Point>,
    pub food: Vec<Food>,
    pub state: State,
    pub update_time: f64,
    pub score: u32,
    pub time: f64,
    pub invisible_walls: Vec<Point>,
}

impl Game {
    pub fn new() -> Game {
        let l = rand_level();
        Game {
            snake: l.snake,
            time: UPDATE_TIME,
            update_time: UPDATE_TIME,
            state: State::Playing,
            walls: l.walls,
            invisible_walls: l.invisible_walls,
            food: vec![],
            score: 0,
        }
    }

    pub fn render(&mut self, context: Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        let transform = context.transform.trans(BORDER, BORDER);
        
        if self.state == State::GameOver {
            clear(color::hex("220011"), graphics);

            text::Text::new_color(color::hex("11AAFF"), 32).draw(
                "Game Over! Press 'R' to restart",
                glyphs,
                &context.draw_state,
                context.transform.trans(10.0, 100.0),
                graphics
            ).unwrap();

            return;
        }

        clear(color::hex("001122"), graphics);

        for ref mut food in &self.food {
            food.render(transform, graphics);
        }

        self.snake.render(transform, graphics);

        for walls in &self.walls {
            rectangle(
                color::hex("002951"),
                rectangle::square(walls.x as f64 * TILE_SIZE, walls.y as f64 * TILE_SIZE, TILE_SIZE),
                transform,
                graphics,
            );
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            State::Paused | State::GameOver => return,
            _ => {}
        }

        self.time += dt;

        if self.time > self.update_time {
            self.time -= self.update_time;
            Snake::update(self);
            Food::update(self);
        }
    }

    pub fn key_press(&mut self, key: Key) {
        match (key, self.state) {
            (Key::R, _) => {
                let l = rand_level();
                self.snake = l.snake;
                self.state = State::Playing;
                self.time = UPDATE_TIME;
                self.update_time = UPDATE_TIME;
                self.walls = l.walls;
                self.invisible_walls = l.invisible_walls;
                self.food = vec![];
                self.score = 0;
                return;
            }
            (Key::P, State::Playing) => {
                self.state = State::Paused;
            }
            (Key::P, State::Paused) => {
                self.state = State::Playing;
            }
            _ => {
                self.snake.key_press(key);
            }
        };
    }
}
