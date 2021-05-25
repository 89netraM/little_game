use std::time::Duration;

use minifb::Window;
use raqote::DrawTarget;

pub trait GameObject {
	fn update(
		&mut self,
		window: &Window,
		dt: &mut DrawTarget,
		game_time: &Duration,
		delta_time: &Duration,
	) -> Result<(), String>;
}
