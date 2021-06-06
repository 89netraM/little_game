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

use self::{game::GameState, meshes::init_meshes, textures::init_textures};

fn main() {
	let mut window = Window::new_with_size("Lazer aMAZEing", 1280, 800);
	window.hide_cursor(true);
	window.set_cursor_grab(true);
	init_textures();
	init_meshes();

	let state = GameState::new(&mut window);
	window.render_loop(state);
}
