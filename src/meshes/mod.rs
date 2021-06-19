use std::{cell::RefCell, path::Path, rc::Rc};

use kiss3d::{
	loader::obj,
	nalgebra::{Translation3, UnitQuaternion, Vector3},
	resource::MeshManager,
	scene::SceneNode,
};
use rand::{rngs::StdRng, Rng};

use super::{rng::rng_for_maze, textures::hsl_to_rgb};

const KEY: &str = "key";
const LOCK: &str = "lock";
const COIN: &str = "coin";

pub type MeshGenerator = dyn FnOnce(&mut SceneNode) -> SceneNode;

#[derive(PartialEq, Eq)]
pub enum ItemKind {
	Key,
	Lock,
	Coin,
}

pub fn init_meshes() {
	MeshManager::get_global_manager(add_meshes);
}

fn add_meshes(manager: &mut MeshManager) {
	manager.add(
		Rc::new(RefCell::new(
			obj::parse(include_str!("./key.obj"), Path::new(""), KEY)
				.swap_remove(0)
				.1,
		)),
		KEY,
	);
	manager.add(
		Rc::new(RefCell::new(
			obj::parse(include_str!("./lock.obj"), Path::new(""), LOCK)
				.swap_remove(0)
				.1,
		)),
		LOCK,
	);
	manager.add(
		Rc::new(RefCell::new(
			obj::parse(include_str!("./coin.obj"), Path::new(""), COIN)
				.swap_remove(0)
				.1,
		)),
		COIN,
	);
}

pub fn generate_key(parent: &mut SceneNode, seed: u64, position: (i64, i64)) -> SceneNode {
	let mut rng: StdRng = rng_for_maze(seed, position);
	let (r, g, b) = hsl_to_rgb(rng.gen(), 0.5, 0.5);

	let mut group = parent.add_group();
	let mut key = group
		.add_geom_with_name(KEY, Vector3::new(1.0, 1.0, 1.0))
		.unwrap();
	key.prepend_to_local_rotation(&UnitQuaternion::from_axis_angle(
		&Vector3::x_axis(),
		std::f32::consts::PI * -0.4,
	));

	let mut position = -0.285;
	let mut odds = 1.0;
	while position <= -0.12 && rng.gen::<f32>() < odds {
		let height = rng.gen_range(0.01..0.075);
		let width = rng.gen_range(0.01..0.05);
		let offset = rng.gen_range(0.0..0.05);
		let mut tooth = key.add_cube(0.05, height, width);
		tooth.prepend_to_local_translation(&Translation3::new(
			0.0,
			-(0.015 + height / 2.0),
			position + offset + width / 2.0,
		));
		position += offset + width;

		odds *= 0.75;
	}

	key.set_color(r, g, b);
	key.set_material_with_name("pixel");

	group
}

pub fn generate_lock(parent: &mut SceneNode) -> SceneNode {
	let mut lock = parent
		.add_geom_with_name(LOCK, Vector3::new(0.25, 0.25, 0.25))
		.unwrap();
	lock.set_color(0.75, 0.84, 0.88);
	lock.set_material_with_name("pixel");
	lock
}

pub fn generate_coin(parent: &mut SceneNode) -> SceneNode {
	let mut coin = parent
		.add_geom_with_name(COIN, Vector3::new(0.1, 0.1, 0.1))
		.unwrap();
	coin.set_color(0.99, 0.8, 0.33);
	coin.set_material_with_name("pixel");
	coin
}
