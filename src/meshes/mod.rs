use super::textures::hsl_to_rgb;
use rand::{rngs::StdRng, Rng};
use std::{cell::RefCell, path::Path, rc::Rc};

use kiss3d::{
	loader::obj,
	nalgebra::{Translation3, UnitQuaternion, Vector3},
	resource::MeshManager,
	scene::SceneNode,
};

use super::rng::rng_for_maze;

pub fn init_meshes() {
	MeshManager::get_global_manager(add_meshes);
}

fn add_meshes(manager: &mut MeshManager) {
	manager.add(
		Rc::new(RefCell::new(
			obj::parse(include_str!("./key.obj"), Path::new(""), "key")
				.swap_remove(0)
				.1,
		)),
		"key",
	);
}

pub fn generate_key(parent: &mut SceneNode, seed: u64, position: (i64, i64)) -> SceneNode
{
	let mut rng: StdRng = rng_for_maze(seed, position);
	let (r, g, b) = hsl_to_rgb(rng.gen(), 0.5, 0.5);

	let mut key = parent
		.add_geom_with_name("key", Vector3::new(1.0, 1.0, 1.0))
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

	key
}
