/* Copyright (C) 2015 by Alexandru Cojocaru */

/* This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>. */

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate gfx_device_gl;

use std::collections::VecDeque;

use graphics::*;
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use piston::input::UpdateEvent;
use piston_window::G2d;
use piston_window::WindowSettings;
use piston_window::*;
use piston_window::Glyphs;
use rand::Rng;

const BOARD_WIDTH: i8 = 15;
const BOARD_HEIGHT: i8 = 15;
const TILE_SIZE: f64 = 50.0;
const UPDATE_TIME: f64 = 0.15;

#[derive(PartialEq, Copy, Clone)]
enum State {
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: i8,
    y: i8,
}

struct Snake {
    tail: VecDeque<Point>,
    keys: VecDeque<Key>,
    last_pressed: Key,
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
    fn new(tail: VecDeque<Point>, key: Key) -> Snake {
        Snake {
            tail: tail,
            keys: VecDeque::new(),
            last_pressed: key,
        }
    }
    fn render(&self, t: math::Matrix2d, gfx: &mut G2d) {
        for p in self.tail.iter() {
            rectangle(
                color::hex("8ba673"),
                rectangle::square(p.x as f64 * TILE_SIZE, p.y as f64 * TILE_SIZE, TILE_SIZE),
                t,
                gfx,
            );
        }
    }

    fn key_press(&mut self, key: Key) {
        use piston::input::keyboard::Key::*;
        match key {
            Right | Down | Left | Up if reverse_direction(key) != self.last_pressed => {
                self.keys.push_back(key);
                self.last_pressed = key;
            }
            _ => {}
        }
    }

    fn mv(g: &mut Game, dtxy: Point) {
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

    fn update(g: &mut Game) {
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

    fn collides(&self, xy: Point) -> bool {
        self.tail.iter().any(|t| *t == xy)
    }
}

#[derive(PartialEq)]
enum FoodType {
    Apple,
    Candy,
}

struct Food {
    food_type: FoodType,
    xy: Point,
    score: u32,
    life_time: u32,
    lived_time: u32,
}

impl Food {
    fn new(t: FoodType, xy: Point, s: u32, lt: u32, probability: f64) -> Option<Food> {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0, 100.0) < probability {
            Some(Food {
                food_type: t,
                xy: xy,
                score: s,
                life_time: lt,
                lived_time: 0,
            })
        } else {
            None
        }
    }

    fn genxy(g: &Game) -> Point {
        loop {
            let mut rng = rand::thread_rng();
            let xy = Point {
                x: rng.gen_range(0, BOARD_WIDTH),
                y: rng.gen_range(0, BOARD_HEIGHT),
            };

            if !(g.snake.tail.iter().any(|t| *t == xy)
                || g.food.iter().any(|f| f.xy == xy)
                || g.walls.iter().any(|w| *w == xy)
                || g.invisible_walls.iter().any(|w| *w == xy))
            {
                return xy;
            }
        }
    }

    fn update(g: &mut Game) {
        if !g.food.iter().any(|f| f.food_type == FoodType::Apple) {
            if let Some(f) = Food::new(FoodType::Apple, Food::genxy(g), 10, 45, 100.0) {
                g.food.push(f)
            }
        }

        if !g.food.iter().any(|f| f.food_type == FoodType::Candy) {
            if let Some(f) = Food::new(FoodType::Candy, Food::genxy(g), 50, 15, 1.0) {
                g.food.push(f)
            }
        }
        for i in 0..g.food.len() {
            g.food[i].lived_time += 1;
            if g.food[i].lived_time > g.food[i].life_time {
                g.food.swap_remove(i);
                break;
            }
        }
    }

    fn render(&self, t: math::Matrix2d, gfx: &mut G2d) {
        if self.life_time - self.lived_time < 6 && self.lived_time % 2 == 0 {
            return;
        }

        let color = match self.food_type {
            FoodType::Apple => color::hex("b83e3e"),
            FoodType::Candy => color::hex("b19d46"),
        };

        rectangle(
            color,
            rectangle::square(
                self.xy.x as f64 * TILE_SIZE,
                self.xy.y as f64 * TILE_SIZE,
                TILE_SIZE,
            ),
            t,
            gfx,
        );
    }
}

macro_rules! walls {
    ( $( $x:expr, $y:expr ),* ) => {
        {
            vec![
            $(
                Point{x:$x, y:$y},
            )*
            ]
        }
    };
}

struct Level {
    snake: Snake,
    walls: Vec<Point>,
    invisible_walls: Vec<Point>,
}

fn level1() -> Level {
    let w = walls![
        1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12, 0, 13, 0, 14, 1, 14, 2,
        14, 3, 14, 4, 14, 5, 14, 6, 14, 8, 14, 9, 14, 10, 14, 11, 14, 12, 14, 13, 1, 14, 2, 14, 3,
        14, 4, 14, 5, 14, 6, 14, 8, 14, 9, 14, 10, 14, 11, 14, 12, 14, 13, 14, 0, 1, 0, 2, 0, 3, 0,
        4, 0, 5, 0, 6, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12, 0, 13, 7, 7
    ];

    let iw = walls![0, 0, 7, 0, 14, 0, 14, 7, 14, 14, 7, 14, 0, 14, 0, 7];

    let mut s = VecDeque::new();
    s.push_back(Point { x: 2, y: 3 });
    s.push_back(Point { x: 2, y: 2 });
    s.push_back(Point { x: 2, y: 1 });

    Level {
        snake: Snake::new(s, Key::Down),
        walls: w,
        invisible_walls: iw,
    }
}

fn level2() -> Level {
    let w = walls![
        2, 2, 3, 3, 4, 4, 5, 5, 7, 7, 9, 9, 10, 10, 11, 11, 12, 12, 12, 2, 11, 3, 10, 4, 9, 5, 7,
        7, 5, 9, 4, 10, 3, 11, 2, 12, 0, 7, 7, 0, 14, 7, 7, 14
    ];
    let iw = walls![];

    let mut s = VecDeque::new();
    s.push_back(Point { x: 2, y: 3 });
    s.push_back(Point { x: 2, y: 2 });
    s.push_back(Point { x: 2, y: 1 });

    Level {
        snake: Snake::new(s, Key::Down),
        walls: w,
        invisible_walls: iw,
    }
}

fn rand_level() -> Level {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0, 2) {
        0 => level1(),
        1 => level2(),
        _ => panic!(""),
    }
}

struct Game {
    snake: Snake,
    time: f64,
    update_time: f64,
    state: State,
    walls: Vec<Point>,
    invisible_walls: Vec<Point>,
    food: Vec<Food>,
    score: u32,
}

impl Game {
    fn new() -> Game {
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

    fn render(&mut self, context: Context, gfx: &mut G2d, glyphs: &mut Glyphs) {
        let transform = context.transform;
        
        if self.state == State::GameOver {
            clear(color::hex("220011"), gfx);

            text::Text::new_color(color::hex("11AAFF"), 32).draw(
                "Game Over! Press 'R' to restart",
                glyphs,
                &context.draw_state,
                context.transform.trans(10.0, 100.0),
                gfx
            ).unwrap();

            return;
        }

        clear(color::hex("001122"), gfx);

        for ref mut food in &self.food {
            food.render(transform, gfx);
        }

        self.snake.render(transform, gfx);

        for walls in &self.walls {
            rectangle(
                color::hex("002951"),
                rectangle::square(walls.x as f64 * TILE_SIZE, walls.y as f64 * TILE_SIZE, TILE_SIZE),
                transform,
                gfx,
            );
        }
    }

    fn update(&mut self, dt: f64) {
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

    fn key_press(&mut self, key: Key) {
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

fn main() {
    let window_width : u32 = BOARD_WIDTH as u32 * 50;
    let window_height : u32 = BOARD_HEIGHT as u32 * 50;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (window_width, window_height))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
        
    println!("{:?}", assets);

    let mut game = Game::new();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics, device| {
            game.render(context, graphics, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        if let Some(args) = e.update_args() {
            game.update(args.dt);
        }

        if let Some(Keyboard(key)) = e.press_args() {
            game.key_press(key)
        };
    }
}
