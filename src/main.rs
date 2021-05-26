mod game_objects;

use std::{
	io::{stdin, stdout, Write},
	time::Instant,
};

use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, SolidSource};

use self::game_objects::{Action, GameObject, Player, Ring};

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
	let mut window = Window::new(TITLE, GAME_SIZE, GAME_SIZE, WindowOptions::default())
		.map_err(|_| "Could not create a window.".to_string())?;

	let size = window.get_size();
	let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
	let start = Instant::now();
	let mut previous = Instant::now();

	let mut game_objects: Vec<Box<dyn GameObject>> =
		vec![Box::new(Ring::default()), Box::new(Player::default())];

	while window.is_open() {
		dt.clear(SolidSource::from_unpremultiplied_argb(
			0xff, 0xff, 0xff, 0xff,
		));

		let game_time = start.elapsed();
		let delta_time = previous.elapsed();

		let mut actions = Vec::new();
		for game_object in &mut game_objects {
			match game_object.update(&window, &mut dt, &game_time, &delta_time)? {
				Action::Continue() => {}
				a => actions.push(a),
			}
		}
		for action in actions {
			match action {
				Action::Add(to_adds) => {
					for to_add in to_adds {
						game_objects.push(to_add);
					}
				}
				Action::Remove(to_removes) => {
					game_objects = game_objects
						.into_iter()
						.filter(|o| !to_removes.contains(&o.id()))
						.collect();
				}
				Action::Continue() => {}
			}
		}

		previous = Instant::now();

		window
			.update_with_buffer(dt.get_data(), size.0, size.1)
			.map_err(|_| "Could not update window frame. No idea what's going one.".to_string())?;
	}

	Ok(())
}
