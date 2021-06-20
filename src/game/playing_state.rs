use std::{
	collections::{HashMap, HashSet},
	f32,
};

use instant::Instant;
#[cfg(target_arch = "wasm32")]
use kiss3d::nalgebra::Vector2;
use kiss3d::{
	conrod::{
		color::Colorable,
		position::Positionable,
		widget::{self, Widget},
		widget_ids,
	},
	event::{Action, Key},
	nalgebra::{distance, Point3, Translation3, UnitQuaternion, Vector3},
	scene::SceneNode,
	window::Window,
};
use rand::{rngs::StdRng, Rng};

#[cfg(target_arch = "wasm32")]
use super::super::js::{get_cursor_movement, get_focus, hide_cursor, JsVector2};
use super::{
	super::{
		camera::FirstPerson,
		map::{Direction, Map, Position, ROOM_CENTER, ROOM_SIZE},
		meshes::{generate_coin, generate_key, generate_lock, ItemKind, MeshGenerator},
		rng::{rand_for_border_walls, rand_for_key, rng_for_maze},
		text::generate_name,
		textures::hsl_to_rgb,
	},
	monster::Monster,
	wall::Wall,
	CamerasEffectRenderer,
	InnerGameState,
	MouseAction,
	MouseButtons,
};

type Chunks = HashMap<
	(i64, i64),
	(
		Vec<Wall>,
		SceneNode,
		Option<(SceneNode, ItemKind)>,
		Vec<(SceneNode, Monster)>,
	),
>;

pub struct PlayingState {
	camera: FirstPerson,
	seed: u64,
	start_time: Instant,
	ui_ids: UiIds,
	chunks: Chunks,
	position: (i64, i64),
	section_name: String,
	section_name_start_time: Instant,
	has_key: bool,
	collected_items: HashSet<(i64, i64)>,
}

pub struct SavedPlayingState {
	camera_eye: Point3<f32>,
	camera_at: Point3<f32>,
	seed: u64,
	position: (i64, i64),
	has_key: bool,
	collected_items: HashSet<(i64, i64)>,
}

impl PlayingState {
	pub fn new(window: &mut Window, seed: u64) -> Self {
		let position = (0, 0);
		Self {
			camera: FirstPerson::new(Point3::new(0.0, 0.25, 0.0), Point3::new(0.0, 0.25, -1.0)),
			seed,
			start_time: Instant::now(),
			ui_ids: UiIds::new(window.conrod_ui_mut().widget_id_generator()),
			chunks: HashMap::new(),
			position,
			section_name: get_section_name(seed, position),
			section_name_start_time: Instant::now(),
			has_key: false,
			collected_items: HashSet::new(),
		}
	}

	pub fn save(&self) -> SavedPlayingState {
		SavedPlayingState {
			camera_eye: *self.camera.eye(),
			camera_at: self.camera.at(),
			seed: self.seed,
			position: self.position,
			has_key: self.has_key,
			collected_items: self.collected_items.clone(),
		}
	}

	pub fn restore(window: &mut Window, save: &SavedPlayingState) -> Self {
		Self {
			camera: FirstPerson::new(save.camera_eye, save.camera_at),
			seed: save.seed,
			start_time: Instant::now(),
			ui_ids: UiIds::new(window.conrod_ui_mut().widget_id_generator()),
			chunks: HashMap::new(),
			position: save.position,
			section_name: get_section_name(save.seed, save.position),
			section_name_start_time: Instant::now(),
			has_key: save.has_key,
			collected_items: save.collected_items.clone(),
		}
	}
}

impl InnerGameState for PlayingState {
	fn init(&mut self, window: &mut Window) {
		#[cfg(target_arch = "wasm32")]
		{
			hide_cursor(true);
		}
		#[cfg(not(target_arch = "wasm32"))]
		{
			window.hide_cursor(true);
			let size = window.size();
			window.set_cursor_position(size.x as f64 / 2.0, size.y as f64 / 2.0);
		}
		update_chunks(
			self.seed,
			self.position,
			window,
			&mut self.chunks,
			&self.collected_items,
		);
	}

	fn step(
		&mut self,
		window: &mut Window,
		mouse_buttons: &MouseButtons,
	) -> Option<Box<dyn InnerGameState>> {
		#[cfg(target_arch = "wasm32")]
		if !get_focus() {
			return Some(Box::new(super::PauseState::new(window, self.save())));
		}
		#[cfg(not(target_arch = "wasm32"))]
		if window.get_key(Key::Escape) == Action::Press {
			return Some(Box::new(super::PauseState::new(window, self.save())));
		}

		#[cfg(target_arch = "wasm32")]
		{
			if let Ok(cursor_movement) = get_cursor_movement().into_serde::<JsVector2>() {
				self.camera.handle_left_button_displacement(&Vector2::new(
					cursor_movement.x,
					cursor_movement.y,
				));
			}
		}

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
				update_chunks(
					self.seed,
					position,
					window,
					&mut self.chunks,
					&self.collected_items,
				);

				self.section_name_start_time = Instant::now();
				self.section_name = get_section_name(self.seed, position);
			}

			for wall in &self.chunks.get(&position).unwrap().0 {
				wall.push_back(&mut next_camera_eye);
			}
			self.camera.set_eye(next_camera_eye);

			self.position = position;
		}

		let item_turn =
			UnitQuaternion::from_axis_angle(&Vector3::y_axis(), f32::consts::PI / 120.0);
		let item_float = Translation3::new(
			0.0,
			self.start_time.elapsed().as_secs_f32().sin() * 0.0025,
			0.0,
		);
		for item in self.chunks.iter_mut() {
			if let Some((i, _)) = &mut item.1 .2 {
				i.prepend_to_local_rotation(&item_turn);
				i.append_translation(&item_float);
			}

			for (node, monster) in item.1 .3.iter_mut() {
				node.set_visible(monster.update(distance(self.camera.eye(), &{
					let monster_translation = node.data().local_translation();
					Point3::new(
						monster_translation.x,
						monster_translation.y,
						monster_translation.z,
					)
				})));
			}
		}

		let mut action_text = None;
		if let Some((_, _, elem, _)) = self.chunks.get_mut(&self.position) {
			if let Some((item, kind)) = elem {
				if distance(self.camera.eye(), &{
					let item_translation = item.data().local_translation();
					Point3::new(item_translation.x, item_translation.y, item_translation.z)
				}) < MAZE_SIZE_HALF
				{
					let lmb_pressed = mouse_buttons.lmb == MouseAction::Pressed;
					if kind == &ItemKind::Lock && self.has_key {
						if lmb_pressed {
							return Some(Box::new(super::EndState::new(
								window,
								self.collected_items.len() - self.has_key as usize,
							)));
						} else {
							action_text = Some("Press LMB to unlock and escape");
						}
					} else if kind == &ItemKind::Key {
						if lmb_pressed {
							self.has_key = true;
							self.collected_items.insert(self.position);
							window.remove_node(item);
							*elem = None;
						} else {
							action_text = Some("Press LMB to collect key");
						}
					} else if kind == &ItemKind::Coin {
						if lmb_pressed {
							self.collected_items.insert(self.position);
							window.remove_node(item);
							*elem = None;
						} else {
							action_text = Some("Press LMB to collect coin");
						}
					}
				}
			}
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

		if let Some(t) = action_text {
			widget::Text::new(t)
				.font_size(20)
				.rgba(1.0, 1.0, 1.0, 1.0)
				.bottom_right_with_margin(50.0)
				.right_justify()
				.set(self.ui_ids.action_text, &mut ui)
		}

		widget::Text::new(&{
			let coins = self.collected_items.len() - self.has_key as usize;
			if coins == 0 {
				"".to_string()
			} else {
				format!("Coins collected: {}", coins)
			}
		})
		.font_size(20)
		.rgba(1.0, 1.0, 1.0, 1.0)
		.bottom_left_with_margin(50.0)
		.set(self.ui_ids.coins_collected_text, &mut ui);

		if self.has_key {
			widget::Text::new("Carrying key")
				.font_size(20)
				.rgba(1.0, 1.0, 1.0, 1.0)
				.top_left_with_margins_on(self.ui_ids.coins_collected_text, -25.0, 0.0)
				.set(self.ui_ids.carrying_key_text, &mut ui);
		}

		None
	}

	fn cameras_and_effect_and_renderer(&mut self) -> CamerasEffectRenderer {
		(Some(&mut self.camera), None, None, None)
	}

	fn clean(&mut self, window: &mut Window) {
		#[cfg(target_arch = "wasm32")]
		{
			hide_cursor(false);
		}
		#[cfg(not(target_arch = "wasm32"))]
		{
			window.hide_cursor(false);
		}
		for (_, (_, mut node, item, monsters)) in self.chunks.drain() {
			window.remove_node(&mut node);
			if let Some((mut i, _)) = item {
				window.remove_node(&mut i);
			}
			for (mut node, _) in monsters {
				window.remove_node(&mut node);
			}
		}
	}
}

const TEXT_VISIBLE_SECONDS: f32 = 5.0;

widget_ids! {
	struct UiIds {
		section_name,
		coins_collected_text,
		carrying_key_text,
		action_text,
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
	chunks: &mut Chunks,
	collected_items: &HashSet<(i64, i64)>,
) {
	for (_, (_, mut node, item, monsters)) in chunks
		.drain_filter(|p, _| ((p.0 - position.0).abs() + (p.1 - position.1).abs()) > CHUNK_RANGE)
	{
		window.remove_node(&mut node);
		if let Some((mut i, _)) = item {
			window.remove_node(&mut i);
		}
		for (mut node, _) in monsters {
			window.remove_node(&mut node);
		}
	}

	for y in -CHUNK_RANGE..=CHUNK_RANGE {
		let width = CHUNK_RANGE - y.abs();
		for x in -width..=width {
			let position = (position.0 + x, position.1 + y);
			chunks.entry(position).or_insert_with(|| {
				add_maze(window, seed, position, !collected_items.contains(&position))
			});
		}
	}
}

const MAZE_HEIGHT: f32 = 2.0;
pub const MAZE_SIZE: f32 = 1.75;
pub const MAZE_SIZE_HALF: f32 = MAZE_SIZE / 2.0;
pub const MAZE_CHUNK_SIZE: f32 = (ROOM_SIZE as f32 + 0.5) * MAZE_SIZE;
const MAZE_OFFSET: f32 = ROOM_CENTER as f32 * -MAZE_SIZE;
const MAZE_ABOVE: Translation3<f32> = Translation3::new(-MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_LEFT: Translation3<f32> = Translation3::new(0.0, 0.0, MAZE_SIZE_HALF);
const MAZE_RIGHT: Translation3<f32> = Translation3::new(0.0, 0.0, -MAZE_SIZE_HALF);
const MAZE_BELOW: Translation3<f32> = Translation3::new(MAZE_SIZE_HALF, 0.0, 0.0);
const MAZE_FLOOR: Translation3<f32> = Translation3::new(0.0, -MAZE_HEIGHT / 2.0, 0.0);
const MAZE_CEILING: Translation3<f32> = Translation3::new(0.0, MAZE_HEIGHT / 2.0, 0.0);
const MONSTER_DISTANCE: f32 = 5.0;

fn add_maze(
	window: &mut Window,
	seed: u64,
	position: (i64, i64),
	should_add_item: bool,
) -> (
	Vec<Wall>,
	SceneNode,
	Option<(SceneNode, ItemKind)>,
	Vec<(SceneNode, Monster)>,
) {
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
	let create_monster_quad = |window: &mut Window| -> SceneNode {
		let mut quad = window.add_quad(MAZE_SIZE * 0.408, MAZE_HEIGHT * 0.861, 1, 1);
		let mut quad2 = quad.add_quad(MAZE_SIZE * 0.408, MAZE_HEIGHT * 0.861, 1, 1);
		quad2.prepend_to_local_rotation(&half_turn);
		quad.set_texture_with_name("monster");
		quad.set_material_with_name("pixel");
		quad.append_translation(&Translation3::new(x_offset, 0.0, z_offset));
		quad.set_visible(false);
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
	let mut monsters = Vec::new();
	let wall_up_opening = rand_for_border_walls::<StdRng>(seed, position, Direction::Up, ROOM_SIZE);
	let wall_left_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Left, ROOM_SIZE);
	let wall_right_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Right, ROOM_SIZE);
	let wall_down_opening =
		rand_for_border_walls::<StdRng>(seed, position, Direction::Down, ROOM_SIZE);

	let mut item_translation: Option<Translation3<f32>> = None;
	let item_creator: Option<(Box<MeshGenerator>, (usize, usize), ItemKind)> = {
		if should_add_item {
			if position == (0, 0) {
				Some((
					Box::new(generate_lock),
					(ROOM_CENTER, ROOM_CENTER),
					ItemKind::Lock,
				))
			} else {
				let mut rng: StdRng = rng_for_maze(seed, position);
				let kind = if position == rand_for_key::<StdRng>(seed) {
					ItemKind::Key
				} else {
					ItemKind::Coin
				};
				Some((
					match kind {
						ItemKind::Key => Box::new(move |p| generate_key(p, seed, (0, 0))),
						ItemKind::Coin => Box::new(generate_coin),
						ItemKind::Lock => panic!(),
					},
					(rng.gen_range(0..ROOM_SIZE), rng.gen_range(0..ROOM_SIZE)),
					kind,
				))
			}
		} else {
			None
		}
	};

	for row in 0..ROOM_SIZE {
		for col in 0..ROOM_SIZE {
			let mut grid_group = wall_group.add_group();
			let pos = Position(row, col);
			let grid_translation =
				Translation3::new(row as f32 * MAZE_SIZE, 0.0, col as f32 * -MAZE_SIZE);
			if row == 0 && Some(col) != wall_up_opening {
				let mut quad = create_maze_quad(&mut grid_group);
				quad.prepend_to_local_rotation(&quarter_turn);
				quad.append_translation(&MAZE_ABOVE);

				walls.push(Wall::Horizontal(Point3::new(
					x_offset + row as f32 * MAZE_SIZE - MAZE_SIZE_HALF,
					0.0,
					z_offset + col as f32 * -MAZE_SIZE,
				)));
			}
			if col == 0 && Some(row) != wall_left_opening {
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
				if Some(row) != wall_right_opening {
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
			} else if rng.gen::<f32>()
				< ((position.0 as f32).powi(2) + (position.1 as f32).powi(2)).sqrt()
					/ MONSTER_DISTANCE
			{
				let mut monster = create_monster_quad(window);
				monster.append_translation(&MAZE_RIGHT);
				monster.append_translation(&grid_translation);
				monsters.push((monster, Monster::default()));
			}
			if row + 1 == ROOM_SIZE {
				if Some(col) != wall_down_opening {
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
			} else if rng.gen::<f32>()
				< ((position.0 as f32).powi(2) + (position.1 as f32).powi(2)).sqrt()
					/ MONSTER_DISTANCE
			{
				let mut monster = create_monster_quad(window);
				monster.prepend_to_local_rotation(&three_quarter_turn);
				monster.append_translation(&MAZE_BELOW);
				monster.append_translation(&grid_translation);
				monsters.push((monster, Monster::default()));
			}
			let mut floor = floor_group.add_quad(MAZE_SIZE, MAZE_SIZE, 1, 1);
			floor.prepend_to_local_rotation(&floor_turn);
			floor.append_translation(&MAZE_FLOOR);
			floor.append_translation(&grid_translation);
			let mut ceiling = ceiling_group.add_quad(MAZE_SIZE, MAZE_SIZE, 1, 1);
			ceiling.prepend_to_local_rotation(&ceiling_turn);
			ceiling.append_translation(&MAZE_CEILING);
			ceiling.append_translation(&grid_translation);
			grid_group.append_translation(&grid_translation);
			if item_creator
				.iter()
				.next()
				.map_or(false, |ic| ic.1 .0 == row && ic.1 .1 == col)
			{
				item_translation = Some(grid_translation);
			}
		}
	}

	ceiling_group.set_texture_with_name("ceiling");
	wall_group.set_texture_with_name("wall");
	floor_group.set_texture_with_name("floor");
	group.set_material_with_name("pixel");
	group.set_color(r, g, b);

	(
		walls,
		group,
		item_creator.map(|ic| {
			let mut item = ic.0(window.scene_mut());
			item.append_translation(&Translation3::new(x_offset, -0.1, z_offset));
			item.append_translation(&item_translation.unwrap());
			(item, ic.2)
		}),
		monsters,
	)
}
