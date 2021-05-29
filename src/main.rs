mod camera;
mod map;
mod wall;

use std::f32;

use kiss3d::{
	event::{Action, Key},
	nalgebra::{Point3, Translation3, UnitQuaternion, Vector3},
	scene::SceneNode,
	window::Window,
};

use self::{
	camera::FirstPerson,
	map::{Map, Position},
	wall::Wall,
};

fn main() {
	let mut window = Window::new_with_size("Lazer aMAZEing", 1280, 800);
	window.hide_cursor(true);
	window.set_cursor_grab(true);

	let mut camera = FirstPerson::new(Point3::new(0.0, 0.25, 0.0), Point3::new(0.0, 0.25, -1.0));

	let mut cube = window.add_cube(0.5, 0.5, 0.5);
	cube.set_color(0.0, 1.0, 1.0);
	cube.append_translation(&Translation3::new(MAZE_SIZE * 2.0, -0.25, MAZE_SIZE * -2.0));

	let (walls, _maze) = add_maze(&mut window);

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
const MAZE_ABOVE: Translation3<f32> = Translation3::new(-MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_LEFT: Translation3<f32> = Translation3::new(0.0, 0.0, MAZE_SIZE_HALF);
const MAZE_RIGHT: Translation3<f32> = Translation3::new(0.0, 0.0, -MAZE_SIZE_HALF);
const MAZE_BELOW: Translation3<f32> = Translation3::new(MAZE_SIZE_HALF, 0.0, 0.0);

fn add_maze(window: &mut Window) -> (Vec<Wall>, SceneNode) {
	fn create_maze_quad(parent: &mut SceneNode) -> SceneNode {
		let mut quad = parent.add_quad(MAZE_SIZE, MAZE_HEIGHT, 1, 1);
		quad.set_color(0.0, 0.0, 0.0);
		quad.set_lines_color(Some(Point3::new(1.0, 1.0, 1.0)));
		quad.set_lines_width(5.0);
		quad
	}

	let map = Map::generate_prim(3, 3, Position(0, 0));
	let mut group = window.add_group();
	let mut walls = Vec::new();

	let quarter_turn = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI / 2.0);

	for row in 0..map.rows {
		for col in 0..map.columns {
			let mut grid_group = group.add_group();
			let pos = Position(row, col);
			if row == 0 {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_ABOVE);

				walls.push(Wall::Horizontal(Point3::new(
					row as f32 * MAZE_SIZE - MAZE_SIZE_HALF,
					0.0,
					col as f32 * -MAZE_SIZE,
				)));
			}
			if col == 0 {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.append_translation(&MAZE_LEFT);

				walls.push(Wall::Vertical(Point3::new(
					row as f32 * MAZE_SIZE,
					0.0,
					col as f32 * -MAZE_SIZE + MAZE_SIZE_HALF,
				)));
			}
			if col + 1 == map.columns || map.is_right(&pos) {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.append_translation(&MAZE_RIGHT);

				walls.push(Wall::Vertical(Point3::new(
					row as f32 * MAZE_SIZE,
					0.0,
					col as f32 * -MAZE_SIZE - MAZE_SIZE_HALF,
				)));
			}
			if row + 1 == map.rows || map.is_below(&pos) {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_BELOW);

				walls.push(Wall::Horizontal(Point3::new(
					row as f32 * MAZE_SIZE + MAZE_SIZE_HALF,
					0.0,
					col as f32 * -MAZE_SIZE,
				)));
			}
			grid_group.append_translation(&Translation3::new(
				row as f32 * MAZE_SIZE,
				0.0,
				col as f32 * -MAZE_SIZE,
			));
		}
	}

	(walls, group)
}
