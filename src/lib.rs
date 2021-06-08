#![feature(hash_drain_filter)]

mod camera;
mod game;
mod js;
mod map;
mod meshes;
mod rng;
mod text;
mod textures;

use kiss3d::window::Window;
use wasm_bindgen::prelude::wasm_bindgen;

use self::{
	game::{GameState, InnerGameState, MenuState},
	meshes::init_meshes,
	textures::init_textures,
};

pub const GAME_NAME: &str = "Lazer aMAZEing";

#[wasm_bindgen(start)]
pub fn main() {
	let mut window = Window::new(GAME_NAME);
	init_textures();
	init_meshes();

	let mut menu_state = Box::new(MenuState::new(&mut window));
	menu_state.init(&mut window);
	let state = GameState::new(menu_state);
	window.render_loop(state);
}
