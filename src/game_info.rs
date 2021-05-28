use std::time::Duration;

use minifb::Window;

use super::game_objects::PhysicalBody;

pub struct GameInfo {
	pub window: Window,
	pub bodies: Vec<PhysicalBody>,
	pub ring_radius: f32,
	pub game_time: Duration,
	pub delta_time: Duration,
}
