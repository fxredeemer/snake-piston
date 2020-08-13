use graphics::*;
use piston_window::G2d;
use rand::Rng;

use crate::consts::*;
use crate::point::*;
use crate::game::*;

#[derive(PartialEq)]
pub enum FoodType {
    Apple,
    Candy,
}

pub struct Food {
    food_type: FoodType,
    pub xy: Point,
    pub score: u32,
    life_time: u32,
    lived_time: u32,
}

impl Food {
    pub fn new(t: FoodType, xy: Point, s: u32, lt: u32, probability: f64) -> Option<Food> {
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

    pub fn genxy(g: &Game) -> Point {
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

    pub fn update(g: &mut Game) {
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

    pub fn render(&self, t: math::Matrix2d, gfx: &mut G2d) {
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
