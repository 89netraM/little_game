mod camera;
mod map;
mod rng;
mod textures;
mod wall;

use std::f32;

use kiss3d::{
	event::{Action, Key},
	nalgebra::{Point3, Translation3, UnitQuaternion, Vector3},
	scene::SceneNode,
	window::Window,
};
use rand::rngs::StdRng;

use self::{
	camera::FirstPerson,
	map::{Direction, Map, Position, ROOM_CENTER, ROOM_SIZE},
	rng::rand_for_border_walls,
	textures::init_textures,
	wall::Wall,
};

fn main() {
	let mut window = Window::new_with_size("Lazer aMAZEing", 1280, 800);
	window.hide_cursor(true);
	window.set_cursor_grab(true);
	init_textures();

	let mut camera = FirstPerson::new(Point3::new(0.0, 0.25, 0.0), Point3::new(0.0, 0.25, -1.0));

	let mut cube = window.add_cube(0.5, 0.5, 0.5);
	cube.set_color(0.0, 1.0, 1.0);
	cube.append_translation(&Translation3::new(MAZE_SIZE * 2.0, -0.25, MAZE_SIZE * -2.0));

	let mut walls = Vec::new();
	let _maze_origin = add_maze(&mut window, 0, (0, 0), &mut walls);
	let _maze_up = add_maze(&mut window, 0, (0, -1), &mut walls);
	let _maze_left = add_maze(&mut window, 0, (-1, 0), &mut walls);
	let _maze_right = add_maze(&mut window, 0, (1, 0), &mut walls);
	let _maze_down = add_maze(&mut window, 0, (0, 1), &mut walls);

	let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

	while window.render_with_camera(&mut camera) {
		cube.prepend_to_local_rotation(&rotation);

		let movement = camera.move_dir(
			window.get_key(Key::W) == Action::Press,
			window.get_key(Key::S) == Action::Press,
			window.get_key(Key::D) == Action::Press,
			window.get_key(Key::A) == Action::Press,
		);
		if let Some(dir) = movement {
			let mut next_camera_eye = camera.eye() + dir;
			for wall in &walls {
				wall.push_back(&mut next_camera_eye);
			}
			camera.set_eye(next_camera_eye);
		}
	}
}

const MAZE_HEIGHT: f32 = 2.0;
pub const MAZE_SIZE: f32 = 1.75;
const MAZE_SIZE_HALF: f32 = MAZE_SIZE / 2.0;
const MAZE_OFFSET: f32 = ROOM_CENTER as f32 * -MAZE_SIZE;
const MAZE_ABOVE: Translation3<f32> = Translation3::new(-MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_LEFT: Translation3<f32> = Translation3::new(0.0, 0.0, MAZE_SIZE_HALF);
const MAZE_RIGHT: Translation3<f32> = Translation3::new(0.0, 0.0, -MAZE_SIZE_HALF);
const MAZE_BELOW: Translation3<f32> = Translation3::new(MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_FLOOR: Translation3<f32> = Translation3::new(0.0, -MAZE_HEIGHT / 2.0, 0.0);
const MAZE_CEILING: Translation3<f32> = Translation3::new(0.0, MAZE_HEIGHT / 2.0, 0.0);

fn add_maze(
	window: &mut Window,
	seed: u64,
	position: (i64, i64),
	walls: &mut Vec<Wall>,
) -> SceneNode {
	let half_turn = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI);
	let quarter_turn = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI / 2.0);
	let three_quarter_turn =
		UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI * 1.5);
	let floor_turn = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -f32::consts::PI / 2.0);
	let ceiling_turn = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), f32::consts::PI / 2.0);
	let x_offset = MAZE_OFFSET + (position.1 * ROOM_SIZE as i64) as f32 * MAZE_SIZE;
	let y_offset = -(MAZE_OFFSET + (position.0 * ROOM_SIZE as i64) as f32 * MAZE_SIZE);

	fn create_maze_quad(parent: &mut SceneNode) -> SceneNode {
		let mut quad = parent.add_quad(MAZE_SIZE, MAZE_HEIGHT, 1, 1);
		quad.set_texture_with_name("wall");
		quad.set_material_with_name("pixel");
		quad
	}
	let create_double_maze_quad = |parent: &mut SceneNode| -> SceneNode {
		let mut quad = create_maze_quad(parent);
		let mut quad2 = quad.add_quad(MAZE_SIZE, MAZE_HEIGHT, 1, 1);
		quad2.prepend_to_local_rotation(&half_turn);
		quad2.set_texture_with_name("wall");
		quad2.set_material_with_name("pixel");
		quad
	};

	let map = Map::generate_prim(seed, position);
	let mut group = window.add_group();
	group.append_translation(&Translation3::new(x_offset, 0.0, y_offset));
	let wall_up_opening = rand_for_border_walls::<StdRng>(seed, position, Direction::Up, ROOM_SIZE);
	let wall_left_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Left, ROOM_SIZE);
	let wall_right_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Right, ROOM_SIZE);
	let wall_down_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Down, ROOM_SIZE);

	for row in 0..ROOM_SIZE {
		for col in 0..ROOM_SIZE {
			let mut grid_group = group.add_group();
			let pos = Position(row, col);
			if row == 0 && col != wall_up_opening {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_ABOVE);
			}
			if col == 0 && row != wall_left_opening {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&half_turn);
				quad.append_translation(&MAZE_LEFT);
			}
			if col + 1 == ROOM_SIZE {
				if row != wall_right_opening {
					let mut quad = create_maze_quad(&mut grid_group);
					quad.append_translation(&MAZE_RIGHT);

					walls.push(Wall::Vertical(Point3::new(
						x_offset + row as f32 * MAZE_SIZE,
						0.0,
						y_offset + col as f32 * -MAZE_SIZE - MAZE_SIZE_HALF,
					)));
				}
			} else if map.is_right(&pos) {
				let mut quad = create_double_maze_quad(&mut grid_group);
				quad.append_translation(&MAZE_RIGHT);

				walls.push(Wall::Vertical(Point3::new(
					x_offset + row as f32 * MAZE_SIZE,
					0.0,
					y_offset + col as f32 * -MAZE_SIZE - MAZE_SIZE_HALF,
				)));
			}
			if row + 1 == ROOM_SIZE {
				if col != wall_down_opening {
					let mut quad = create_maze_quad(&mut grid_group);
					quad.prepend_to_local_rotation(&three_quarter_turn);
					quad.append_translation(&MAZE_BELOW);

					walls.push(Wall::Horizontal(Point3::new(
						x_offset + row as f32 * MAZE_SIZE + MAZE_SIZE_HALF,
						0.0,
						y_offset + col as f32 * -MAZE_SIZE,
					)));
				}
			} else if map.is_below(&pos) {
				let mut quad = create_double_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_BELOW);

				walls.push(Wall::Horizontal(Point3::new(
					x_offset + row as f32 * MAZE_SIZE + MAZE_SIZE_HALF,
					0.0,
					y_offset + col as f32 * -MAZE_SIZE,
				)));
			}
			let mut floor = grid_group.add_quad(MAZE_SIZE, MAZE_SIZE, 1, 1);
			floor.prepend_to_local_rotation(&floor_turn);
			floor.append_translation(&MAZE_FLOOR);
			floor.set_texture_with_name("floor");
			floor.set_material_with_name("pixel");
			let mut ceiling = grid_group.add_quad(MAZE_SIZE, MAZE_SIZE, 1, 1);
			ceiling.prepend_to_local_rotation(&ceiling_turn);
			ceiling.append_translation(&MAZE_CEILING);
			ceiling.set_texture_with_name("ceiling");
			ceiling.set_material_with_name("pixel");
			grid_group.append_translation(&Translation3::new(
				row as f32 * MAZE_SIZE,
				0.0,
				col as f32 * -MAZE_SIZE,
			));
		}
	}

	group
}
