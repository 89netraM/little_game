use std::{collections::HashMap, f32};

use instant::Instant;
use kiss3d::{
	conrod::{
		color::Colorable,
		position::Positionable,
		widget::{self, Widget},
		widget_ids,
	},
	event::{Action, Key},
	nalgebra::{Point3, Translation3, UnitQuaternion, Vector3},
	scene::SceneNode,
	window::Window,
};
use rand::{rngs::StdRng, Rng};

use super::{
	super::{
		camera::FirstPerson,
		map::{Direction, Map, Position, ROOM_CENTER, ROOM_SIZE},
		rng::{rand_for_border_walls, rng_for_maze},
		text::generate_name,
		textures::hsl_to_rgb,
	},
	wall::Wall,
	CamerasEffectRenderer,
	InnerGameState,
};

pub struct PlayingState {
	camera: FirstPerson,
	seed: u64,
	ui_ids: UiIds,
	chunks: HashMap<(i64, i64), (Vec<Wall>, SceneNode)>,
	position: (i64, i64),
	section_name: String,
	section_name_start_time: Instant,
}

impl PlayingState {
	pub fn new(window: &mut Window, seed: u64, position: (i64, i64)) -> Self {
		Self {
			camera: FirstPerson::new(Point3::new(0.0, 0.25, 0.0), Point3::new(0.0, 0.25, -1.0)),
			seed,
			ui_ids: UiIds::new(window.conrod_ui_mut().widget_id_generator()),
			chunks: HashMap::new(),
			position,
			section_name: get_section_name(seed, position),
			section_name_start_time: Instant::now(),
		}
	}
}

impl InnerGameState for PlayingState {
	fn init(&mut self, window: &mut Window) {
		window.hide_cursor(true);
		let size = window.size();
		window.set_cursor_position(size.x as f64 / 2.0, size.y as f64 / 2.0);
		update_chunks(self.seed, self.position, window, &mut self.chunks);
	}

	fn step(&mut self, window: &mut Window) -> Option<Box<dyn InnerGameState>> {
		let movement = self.camera.move_dir(
			window.get_key(Key::W) == Action::Press,
			window.get_key(Key::S) == Action::Press,
			window.get_key(Key::D) == Action::Press,
			window.get_key(Key::A) == Action::Press,
		);
		if let Some(dir) = movement {
			let mut next_camera_eye = self.camera.eye() + dir;

			let position = (
				(-self.camera.eye().z / MAZE_SIZE / ROOM_SIZE as f32).round() as i64,
				(self.camera.eye().x / MAZE_SIZE / ROOM_SIZE as f32).round() as i64,
			);
			if position != self.position {
				update_chunks(self.seed, position, window, &mut self.chunks);

				self.section_name_start_time = Instant::now();
				self.section_name = get_section_name(self.seed, position);
			}

			for wall in &self.chunks.get(&position).unwrap().0 {
				wall.push_back(&mut next_camera_eye);
			}
			self.camera.set_eye(next_camera_eye);

			self.position = position;
		}

		let mut ui = window.conrod_ui_mut().set_widgets();

		let text_time = self.section_name_start_time.elapsed().as_secs_f32();
		if text_time < TEXT_VISIBLE_SECONDS {
			widget::Text::new(&self.section_name)
				.font_size(50)
				.rgba(
					1.0,
					1.0,
					1.0,
					(1.5625 - (2.5 * text_time / TEXT_VISIBLE_SECONDS - 1.25).powi(2)).min(1.0),
				)
				.mid_top_with_margin(100.0)
				.center_justify()
				.set(self.ui_ids.section_name, &mut ui);
		}

		None
	}

	fn cameras_and_effect_and_renderer(&mut self) -> CamerasEffectRenderer {
		(Some(&mut self.camera), None, None, None)
	}

	fn clean(&mut self, window: &mut Window) {
		window.hide_cursor(false);
		for (_, (_, mut node)) in self.chunks.drain() {
			window.remove_node(&mut node);
		}
	}
}

const TEXT_VISIBLE_SECONDS: f32 = 5.0;

widget_ids! {
	struct UiIds {
		section_name,
	}
}

fn get_section_name(seed: u64, position: (i64, i64)) -> String {
	let mut name = "– ".to_string();
	generate_name(seed, position, &mut name);
	name.push_str(" –");
	name
}

const CHUNK_RANGE: i64 = 2;

fn update_chunks(
	seed: u64,
	position: (i64, i64),
	window: &mut Window,
	chunks: &mut HashMap<(i64, i64), (Vec<Wall>, SceneNode)>,
) {
	for (_, (_, mut node)) in chunks
		.drain_filter(|p, _| ((p.0 - position.0).abs() + (p.1 - position.1).abs()) > CHUNK_RANGE)
	{
		window.remove_node(&mut node);
	}

	for y in -CHUNK_RANGE..=CHUNK_RANGE {
		let width = CHUNK_RANGE - y.abs();
		for x in -width..=width {
			let position = (position.0 + x, position.1 + y);
			chunks
				.entry(position)
				.or_insert_with(|| add_maze(window, seed, position));
		}
	}
}

const MAZE_HEIGHT: f32 = 2.0;
pub const MAZE_SIZE: f32 = 1.75;
pub const MAZE_SIZE_HALF: f32 = MAZE_SIZE / 2.0;
const MAZE_OFFSET: f32 = ROOM_CENTER as f32 * -MAZE_SIZE;
const MAZE_ABOVE: Translation3<f32> = Translation3::new(-MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_LEFT: Translation3<f32> = Translation3::new(0.0, 0.0, MAZE_SIZE_HALF);
const MAZE_RIGHT: Translation3<f32> = Translation3::new(0.0, 0.0, -MAZE_SIZE_HALF);
const MAZE_BELOW: Translation3<f32> = Translation3::new(MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_FLOOR: Translation3<f32> = Translation3::new(0.0, -MAZE_HEIGHT / 2.0, 0.0);
const MAZE_CEILING: Translation3<f32> = Translation3::new(0.0, MAZE_HEIGHT / 2.0, 0.0);

fn add_maze(window: &mut Window, seed: u64, position: (i64, i64)) -> (Vec<Wall>, SceneNode) {
	let half_turn = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI);
	let quarter_turn = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI / 2.0);
	let three_quarter_turn =
		UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI * 1.5);
	let floor_turn = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -f32::consts::PI / 2.0);
	let ceiling_turn = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), f32::consts::PI / 2.0);
	let x_offset = MAZE_OFFSET + (position.1 * ROOM_SIZE as i64) as f32 * MAZE_SIZE;
	let z_offset = -(MAZE_OFFSET + (position.0 * ROOM_SIZE as i64) as f32 * MAZE_SIZE);

	fn create_maze_quad(parent: &mut SceneNode) -> SceneNode {
		parent.add_quad(MAZE_SIZE, MAZE_HEIGHT, 1, 1)
	}
	let create_double_maze_quad = |parent: &mut SceneNode| -> SceneNode {
		let mut quad = create_maze_quad(parent);
		let mut quad2 = quad.add_quad(MAZE_SIZE, MAZE_HEIGHT, 1, 1);
		quad2.prepend_to_local_rotation(&half_turn);
		quad
	};

	let mut rng: StdRng = rng_for_maze(seed, position);
	let (r, g, b) = hsl_to_rgb(rng.gen(), 0.5, 0.5);
	let map = Map::generate_prim(&mut rng);
	let mut walls = Vec::new();
	let mut group = window.add_group();
	group.append_translation(&Translation3::new(x_offset, 0.0, z_offset));
	let mut ceiling_group = group.add_group();
	let mut wall_group = group.add_group();
	let mut floor_group = group.add_group();
	let wall_up_opening = rand_for_border_walls::<StdRng>(seed, position, Direction::Up, ROOM_SIZE);
	let wall_left_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Left, ROOM_SIZE);
	let wall_right_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Right, ROOM_SIZE);
	let wall_down_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Down, ROOM_SIZE);

	for row in 0..ROOM_SIZE {
		for col in 0..ROOM_SIZE {
			let mut grid_group = wall_group.add_group();
			let pos = Position(row, col);
			if row == 0 && col != wall_up_opening {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_ABOVE);

				walls.push(Wall::Horizontal(Point3::new(
					x_offset + row as f32 * MAZE_SIZE - MAZE_SIZE_HALF,
					0.0,
					z_offset + col as f32 * -MAZE_SIZE,
				)));
			}
			if col == 0 && row != wall_left_opening {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&half_turn);
				quad.append_translation(&MAZE_LEFT);

				walls.push(Wall::Vertical(Point3::new(
					x_offset + row as f32 * MAZE_SIZE,
					0.0,
					z_offset + col as f32 * -MAZE_SIZE + MAZE_SIZE_HALF,
				)));
			}
			if col + 1 == ROOM_SIZE {
				if row != wall_right_opening {
					let mut quad = create_maze_quad(&mut grid_group);
					quad.append_translation(&MAZE_RIGHT);

					walls.push(Wall::Vertical(Point3::new(
						x_offset + row as f32 * MAZE_SIZE,
						0.0,
						z_offset + col as f32 * -MAZE_SIZE - MAZE_SIZE_HALF,
					)));
				}
			} else if map.is_right(&pos) {
				let mut quad = create_double_maze_quad(&mut grid_group);
				quad.append_translation(&MAZE_RIGHT);

				walls.push(Wall::Vertical(Point3::new(
					x_offset + row as f32 * MAZE_SIZE,
					0.0,
					z_offset + col as f32 * -MAZE_SIZE - MAZE_SIZE_HALF,
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
						z_offset + col as f32 * -MAZE_SIZE,
					)));
				}
			} else if map.is_below(&pos) {
				let mut quad = create_double_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_BELOW);

				walls.push(Wall::Horizontal(Point3::new(
					x_offset + row as f32 * MAZE_SIZE + MAZE_SIZE_HALF,
					0.0,
					z_offset + col as f32 * -MAZE_SIZE,
				)));
			}
			let grid_translation =
				Translation3::new(row as f32 * MAZE_SIZE, 0.0, col as f32 * -MAZE_SIZE);
			let mut floor = floor_group.add_quad(MAZE_SIZE, MAZE_SIZE, 1, 1);
			floor.prepend_to_local_rotation(&floor_turn);
			floor.append_translation(&MAZE_FLOOR);
			floor.append_translation(&grid_translation);
			let mut ceiling = ceiling_group.add_quad(MAZE_SIZE, MAZE_SIZE, 1, 1);
			ceiling.prepend_to_local_rotation(&ceiling_turn);
			ceiling.append_translation(&MAZE_CEILING);
			ceiling.append_translation(&grid_translation);
			grid_group.append_translation(&grid_translation);
		}
	}

	ceiling_group.set_texture_with_name("ceiling");
	ceiling_group.set_material_with_name("pixel");
	wall_group.set_texture_with_name("wall");
	wall_group.set_material_with_name("pixel");
	floor_group.set_texture_with_name("floor");
	floor_group.set_material_with_name("pixel");
	group.set_color(r, g, b);

	(walls, group)
}
