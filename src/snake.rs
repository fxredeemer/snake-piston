
use graphics::*;
use piston::input::keyboard::Key;
use piston_window::G2d;
use std::collections::VecDeque;

use crate::point::*;
use crate::consts::*;
use crate::game::*;
use crate::state::*;

pub struct Snake {
    pub tail: VecDeque<Point>,
    pub keys: VecDeque<Key>,
    pub last_pressed: Key,
}

fn reverse_direction(key: Key) -> Key {
    match key {
        Key::Down => Key::Up,
        Key::Up => Key::Down,
        Key::Left => Key::Right,
        Key::Right => Key::Left,
        other => other,
    }
}

impl Snake {
    pub fn new(tail: VecDeque<Point>, key: Key) -> Snake {
        Snake {
            tail: tail,
            keys: VecDeque::new(),
            last_pressed: key,
        }
    }
    pub fn render(&self, t: math::Matrix2d, gfx: &mut G2d) {
        for p in self.tail.iter() {
            rectangle(
                color::hex("8ba673"),
                rectangle::square(p.x as f64 * TILE_SIZE, p.y as f64 * TILE_SIZE, TILE_SIZE),
                t,
                gfx,
            );
        }
    }

    pub fn key_press(&mut self, key: Key) {
        use piston::input::keyboard::Key::*;
        match key {
            Right | Down | Left | Up if reverse_direction(key) != self.last_pressed => {
                self.keys.push_back(key);
                self.last_pressed = key;
            }
            _ => {}
        }
    }

    pub fn mv(g: &mut Game, dtxy: Point) {
        let mut xy = Point {
            x: g.snake.tail.front().unwrap().x + dtxy.x,
            y: g.snake.tail.front().unwrap().y + dtxy.y,
        };
        if xy.x >= BOARD_WIDTH {
            xy.x = 0;
        } else if xy.x < 0 {
            xy.x = BOARD_WIDTH - 1;
        }

        if xy.y >= BOARD_HEIGHT {
            xy.y = 0;
        } else if xy.y < 0 {
            xy.y = BOARD_HEIGHT - 1;
        }

        if g.walls.iter().any(|w| *w == xy) || g.snake.collides(xy) {
            g.state = State::GameOver;
            println!(
                "### Game Over ###\nScore: {}\nPress R to restart\nPress Esc to quit",
                g.score
            );
            return;
        }

        for i in 0..g.food.len() {
            if g.food[i].xy == xy {
                let f = g.food.swap_remove(i);
                g.score += f.score;
                let xy = *g.snake.tail.front().unwrap();
                g.snake.tail.push_back(xy);
                g.update_time -= 0.002;
                break;
            }
        }

        g.snake.tail.pop_back();
        g.snake.tail.push_front(xy);
    }

    pub fn update(g: &mut Game) {
        use piston::input::keyboard::Key::*;
        if g.snake.keys.is_empty() {
            g.snake.keys.push_back(g.snake.last_pressed);
        }
        let k = g.snake.keys.pop_front().unwrap();
        Snake::mv(
            g,
            match k {
                Right => Point { x: 1, y: 0 },
                Down => Point { x: 0, y: 1 },
                Left => Point { x: -1, y: 0 },
                Up => Point { x: 0, y: -1 },
                _ => panic!("only UP/DOWN/LEFT/UP arrows allowed"),
            },
        )
    }

    pub fn collides(&self, xy: Point) -> bool {
        self.tail.iter().any(|t| *t == xy)
    }
}
