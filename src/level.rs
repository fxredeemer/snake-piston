use piston::input::keyboard::Key;
use rand::Rng;
use std::collections::VecDeque;

use crate::point::*;
use crate::snake::*;

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

pub struct Level {
    pub snake: Snake,
    pub walls: Vec<Point>,
    pub invisible_walls: Vec<Point>,
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

pub fn rand_level() -> Level {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0, 2) {
        0 => level1(),
        1 => level2(),
        _ => panic!(""),
    }
}
