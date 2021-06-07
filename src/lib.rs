#![feature(hash_drain_filter)]

mod camera;
mod game;
mod map;
mod meshes;
mod rng;
mod text;
mod textures;
mod wall;

use kiss3d::window::Window;
use wasm_bindgen::prelude::wasm_bindgen;

use self::{game::GameState, meshes::init_meshes, textures::init_textures};

#[wasm_bindgen(start)]
pub fn main() {
	let mut window = Window::new("Lazer aMAZEing");
	window.hide_cursor(true);
	window.set_cursor_grab(true);
	init_textures();
	init_meshes();

	let state = GameState::new(&mut window);
	window.render_loop(state);
}
