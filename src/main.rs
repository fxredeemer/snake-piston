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

mod food;
mod state;
mod consts;
mod snake;
mod level;
mod game;
mod point;

use crate::consts::*;
use crate::game::*;

use piston::input::Button::Keyboard;
use piston::input::UpdateEvent;
use piston_window::WindowSettings;
use piston_window::*;

fn main() {
    let window_width : u32 = BOARD_WIDTH as u32 * TILE_SIZE as u32 + 2 * BORDER as u32;
    let window_height : u32 = BOARD_HEIGHT as u32 * TILE_SIZE as u32 + 2 * BORDER as u32;

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
