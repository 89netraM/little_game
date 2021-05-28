use euclid::{UnknownUnit, Vector2D};
use raqote::DrawTarget;

use crate::game_info::GameInfo;

static mut OBJECT_ID_COUNTER: usize = 0;

pub fn get_new_object_id() -> usize {
	unsafe {
		let id = OBJECT_ID_COUNTER;
		OBJECT_ID_COUNTER += 1;
		id
	}
}

pub struct PhysicalBody {
	pub id: usize,
	pub pos: Vector2D<f32, UnknownUnit>,
	pub radius: f32,
}

pub trait GameObject {
	fn id(&self) -> usize;

	fn body(&self) -> Option<PhysicalBody>;

	fn update(&mut self, game_info: &GameInfo, dt: &mut DrawTarget) -> Result<Action, String>;
}

pub enum Action {
	Continue(),
	Add(Vec<Box<dyn GameObject>>),
	Remove(Vec<usize>),
}
