mod game_info;
mod game_objects;

use std::{
	io::{stdin, stdout, Write},
	time::{Duration, Instant},
};

use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, SolidSource};

use self::{
	game_info::GameInfo,
	game_objects::{Action, EnemySpawner, GameObject, Player, Ring},
};

const TITLE: &str = "Little Game";
pub const GAME_SIZE: usize = 800;

fn main() {
	if let Err(description) = game() {
		println!("{}", description);
		print!("Press any key to exit...");
		stdout().flush().unwrap();
		stdin().read_line(&mut String::new()).unwrap();
	}
}

fn game() -> Result<(), String> {
	let mut game_objects: Vec<Box<dyn GameObject>> = vec![
		Box::new(Ring::default()),
		Box::new(Player::default()),
		Box::new(EnemySpawner::default()),
	];
	let mut game_info = GameInfo {
		window: Window::new(TITLE, GAME_SIZE, GAME_SIZE, WindowOptions::default())
			.map_err(|_| "Could not create a window.".to_string())?,
		bodies: Vec::new(),
		game_time: Duration::default(),
		delta_time: Duration::default(),
	};

	let size = game_info.window.get_size();
	let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
	let start = Instant::now();
	let mut previous = Instant::now();

	while game_info.window.is_open() {
		dt.clear(SolidSource::from_unpremultiplied_argb(
			0xff, 0xff, 0xff, 0xff,
		));

		game_info.bodies.clear();
		game_info
			.bodies
			.extend(game_objects.iter().filter_map(|o| o.body()));
		game_info.game_time = start.elapsed();
		game_info.delta_time = previous.elapsed();

		let mut actions = Vec::new();
		for game_object in &mut game_objects {
			actions.extend(game_object.update(&game_info, &mut dt)?);
		}
		for action in actions {
			match action {
				Action::Add(to_add) => game_objects.push(to_add),
				Action::Remove(to_remove) => {
					if let Some(i) = game_objects.iter().position(|o| o.id() == to_remove) {
						game_objects.remove(i);
					}
				}
			}
		}

		previous = Instant::now();

		game_info
			.window
			.update_with_buffer(dt.get_data(), size.0, size.1)
			.map_err(|_| "Could not update window frame. No idea what's going one.".to_string())?;
	}

	Ok(())
}
