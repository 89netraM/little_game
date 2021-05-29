mod game_info;
mod game_objects;

use std::{
	io::{stdin, stdout, Write},
	time::{Duration, Instant},
};

use font_kit::{family_name::FamilyName, font::Font, properties::Properties, source::SystemSource};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle};

use self::{
	game_info::GameInfo,
	game_objects::{Action, EnemySpawner, GameObject, Player, Ring},
};

const TITLE: &str = "Little Game";
pub const GAME_SIZE: f32 = 800.0;
const MIN_RING_RADIUS: f32 = GAME_SIZE / 20.0;
const BUTTON_GAP: f32 = 20.0;
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 50.0;

fn main() {
	if let Err(description) = game() {
		println!("{}", description);
		print!("Press any key to exit...");
		stdout().flush().unwrap();
		stdin().read_line(&mut String::new()).unwrap();
	}
}

fn game() -> Result<(), String> {
	let font = SystemSource::new()
		.select_best_match(&[FamilyName::Monospace], &Properties::new())
		.map_err(|_| "Could not find font".to_string())?
		.load()
		.map_err(|_| "Could not load font".to_string())?;
	let mut window = Window::new(
		TITLE,
		GAME_SIZE as usize,
		GAME_SIZE as usize,
		WindowOptions::default(),
	)
	.map_err(|_| "Could not create a window.".to_string())?;
	let size = window.get_size();
	let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

	let mut cont = start(&font, &mut window, &mut dt)?;
	while cont {
		cont = play(&font, &mut window, &mut dt)? && restart(&font, &mut window, &mut dt)?;
	}

	Ok(())
}

fn start(font: &Font, window: &mut Window, dt: &mut DrawTarget) -> Result<bool, String> {
	dt.clear(SolidSource::from_unpremultiplied_argb(
		0xff, 0xff, 0xff, 0xff,
	));
	dt.draw_text(
		&font,
		100.0,
		"Little Game",
		Point::new((GAME_SIZE - 600.0) / 2.0, GAME_SIZE / 2.0),
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0x00, 0xff, 0xff,
		)),
		&DrawOptions::new(),
	);
	dt.draw_text(
		&font,
		20.0,
		"Created by Mårten Åsberg for the 4MB Game Jam",
		Point::new((GAME_SIZE - 490.0) / 2.0, GAME_SIZE - 10.0),
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0x00, 0x00, 0x00,
		)),
		&DrawOptions::new(),
	);
	let restart_button = draw_button_at(
		dt,
		font,
		(GAME_SIZE - BUTTON_WIDTH) / 2.0,
		GAME_SIZE / 2.0 + 60.0,
		"Start",
		110.0,
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0x00, 0xff, 0xff,
		)),
	);
	window
		.update_with_buffer(dt.get_data(), GAME_SIZE as usize, GAME_SIZE as usize)
		.map_err(|_| "Could not update window frame. No idea what's going one.".to_string())?;

	let mut was_mouse_down = false;
	while window.is_open() {
		let is_mouse_down = window.get_mouse_down(MouseButton::Left);
		if was_mouse_down && !is_mouse_down {
			if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
				if restart_button.0 < mouse_pos.0
					&& mouse_pos.0 < restart_button.2
					&& restart_button.1 < mouse_pos.1
					&& mouse_pos.1 < restart_button.3
				{
					return Ok(true);
				}
			}
		}

		was_mouse_down = is_mouse_down;
		window.update();
	}

	Ok(false)
}

fn play(font: &Font, window: &mut Window, dt: &mut DrawTarget) -> Result<bool, String> {
	let mut score = 0;
	let mut have_clicked = false;
	let mut have_moved = false;

	let mut ring = Ring::default();
	let mut game_objects: Vec<Box<dyn GameObject>> = vec![
		Box::new(Player::default()),
		Box::new(EnemySpawner::default()),
	];
	let mut game_info = GameInfo {
		window,
		bodies: Vec::new(),
		ring_radius: ring.radius(),
		game_time: Duration::default(),
		delta_time: Duration::default(),
	};

	let size = game_info.window.get_size();
	let start = Instant::now();
	let mut previous = Instant::now();

	while game_info.window.is_open() && game_info.ring_radius > MIN_RING_RADIUS {
		game_info.delta_time = previous.elapsed();
		previous = Instant::now();
		game_info.bodies.clear();
		game_info
			.bodies
			.extend(game_objects.iter().filter_map(|o| o.body()));
		game_info.ring_radius = ring.radius();
		game_info.game_time = start.elapsed();

		dt.clear(SolidSource::from_unpremultiplied_argb(
			0xff, 0xff, 0xff, 0xff,
		));

		ring.update(&game_info, dt)?;
		let mut actions = Vec::new();
		for game_object in &mut game_objects {
			actions.extend(game_object.update(&game_info, dt)?);
		}
		for action in actions {
			match action {
				Action::Add(to_add) => game_objects.push(to_add),
				Action::Remove(to_remove) => {
					if let Some(i) = game_objects.iter().position(|o| o.id() == to_remove) {
						game_objects.remove(i);
					}
				}
				Action::Shrink() => ring.shrink(),
				Action::Score(s) => score += s,
			}
		}

		dt.draw_text(
			&font,
			30.0,
			&format!("Score: {}", score),
			Point::new(0.0, 30.0),
			&Source::Solid(SolidSource::from_unpremultiplied_argb(
				0xff, 0x00, 0x00, 0x00,
			)),
			&DrawOptions::new(),
		);

		let mut margin_bottom = 5.0;
		if !have_moved {
			if game_info.window.is_key_down(Key::W)
				|| game_info.window.is_key_down(Key::A)
				|| game_info.window.is_key_down(Key::S)
				|| game_info.window.is_key_down(Key::D)
			{
				have_moved = true;
			} else {
				dt.draw_text(
					&font,
					20.0,
					"WASD to move",
					Point::new(0.0, GAME_SIZE - margin_bottom),
					&Source::Solid(SolidSource::from_unpremultiplied_argb(
						0xff, 0x00, 0x00, 0x00,
					)),
					&DrawOptions::new(),
				);
				margin_bottom += 20.0;
			}
		}
		if !have_clicked {
			if game_info.window.get_mouse_down(MouseButton::Left) {
				have_clicked = true;
			} else {
				dt.draw_text(
					&font,
					20.0,
					"LMB to shoot",
					Point::new(0.0, GAME_SIZE - margin_bottom),
					&Source::Solid(SolidSource::from_unpremultiplied_argb(
						0xff, 0x00, 0x00, 0x00,
					)),
					&DrawOptions::new(),
				);
			}
		}

		game_info
			.window
			.update_with_buffer(dt.get_data(), size.0, size.1)
			.map_err(|_| "Could not update window frame. No idea what's going one.".to_string())?;
	}

	Ok(game_info.window.is_open())
}

fn restart(font: &Font, window: &mut Window, dt: &mut DrawTarget) -> Result<bool, String> {
	dt.draw_text(
		&font,
		90.0,
		"Game Over",
		Point::new((GAME_SIZE - 450.0) / 2.0, GAME_SIZE / 2.0),
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0xff, 0x00, 0x00,
		)),
		&DrawOptions::new(),
	);
	let restart_button = draw_button_at(
		dt,
		font,
		GAME_SIZE / 2.0 - (BUTTON_WIDTH + BUTTON_GAP),
		GAME_SIZE / 2.0 + 60.0,
		"Restart",
		150.0,
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0x00, 0xff, 0x00,
		)),
	);
	let exit_button = draw_button_at(
		dt,
		font,
		GAME_SIZE / 2.0 + BUTTON_GAP,
		GAME_SIZE / 2.0 + 60.0,
		"Exit",
		90.0,
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0xff, 0x00, 0x00,
		)),
	);
	window
		.update_with_buffer(dt.get_data(), GAME_SIZE as usize, GAME_SIZE as usize)
		.map_err(|_| "Could not update window frame. No idea what's going one.".to_string())?;

	let mut was_mouse_down = false;
	while window.is_open() {
		let is_mouse_down = window.get_mouse_down(MouseButton::Left);
		if was_mouse_down && !is_mouse_down {
			if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
				if restart_button.0 < mouse_pos.0
					&& mouse_pos.0 < restart_button.2
					&& restart_button.1 < mouse_pos.1
					&& mouse_pos.1 < restart_button.3
				{
					return Ok(true);
				} else if exit_button.0 < mouse_pos.0
					&& mouse_pos.0 < exit_button.2
					&& exit_button.1 < mouse_pos.1
					&& mouse_pos.1 < exit_button.3
				{
					return Ok(false);
				}
			}
		}

		was_mouse_down = is_mouse_down;
		window.update();
	}

	Ok(false)
}

fn draw_button_at(
	dt: &mut DrawTarget,
	font: &Font,
	x: f32,
	y: f32,
	text: &str,
	text_width: f32,
	border_color: &Source,
) -> (f32, f32, f32, f32) {
	dt.fill_rect(
		x,
		y,
		BUTTON_WIDTH,
		BUTTON_HEIGHT,
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0xff, 0xff, 0xff,
		)),
		&DrawOptions::new(),
	);
	let mut pb = PathBuilder::new();
	pb.move_to(x, y);
	pb.line_to(x + BUTTON_WIDTH, y);
	pb.line_to(x + BUTTON_WIDTH, y + BUTTON_HEIGHT);
	pb.line_to(x, y + BUTTON_HEIGHT);
	pb.line_to(x, y);
	dt.stroke(
		&pb.finish(),
		border_color,
		&StrokeStyle::default(),
		&DrawOptions::new(),
	);
	dt.draw_text(
		font,
		40.0,
		text,
		Point::new(
			x + (BUTTON_WIDTH - text_width) / 2.0,
			y + BUTTON_HEIGHT - 10.0,
		),
		&Source::Solid(SolidSource::from_unpremultiplied_argb(
			0xff, 0x00, 0x00, 0x00,
		)),
		&DrawOptions::new(),
	);

	(x, y, x + BUTTON_WIDTH, y + BUTTON_HEIGHT)
}
