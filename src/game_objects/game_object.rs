use std::time::Duration;

use minifb::Window;
use raqote::DrawTarget;

static mut OBJECT_ID_COUNTER: usize = 0;

pub fn get_new_object_id() -> usize {
	unsafe {
		let id = OBJECT_ID_COUNTER;
		OBJECT_ID_COUNTER += 1;
		id
	}
}

pub trait GameObject {
	fn id(&self) -> usize;

	fn update(
		&mut self,
		window: &Window,
		dt: &mut DrawTarget,
		game_time: &Duration,
		delta_time: &Duration,
	) -> Result<Action, String>;
}

pub enum Action {
	Continue(),
	Add(Vec<Box<dyn GameObject>>),
	Remove(Vec<usize>),
}
