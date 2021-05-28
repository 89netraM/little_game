use euclid::{UnknownUnit, Vector2D};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use super::{game_object::get_new_object_id, Action, GameObject, PhysicalBody};
use crate::{game_info::GameInfo, GAME_SIZE};

const SIZE: f32 = GAME_SIZE as f32 / 160.0;
const HALF_SIZE: f32 = SIZE / 2.0;
const FAR_EDGE: f32 = GAME_SIZE as f32 - HALF_SIZE;
const SPEED: f32 = GAME_SIZE as f32 / 1.0;
const CIRCLE_ANGLE: f32 = std::f32::consts::PI * 2.0;

pub struct Bullet {
	id: usize,
	pos: Vector2D<f32, UnknownUnit>,
	heading: Vector2D<f32, UnknownUnit>,
}

impl Bullet {
	pub fn new(pos: Vector2D<f32, UnknownUnit>, heading: Vector2D<f32, UnknownUnit>) -> Self {
		Bullet {
			id: get_new_object_id(),
			pos,
			heading,
		}
	}

	fn update(&mut self, game_info: &GameInfo) -> Action {
		self.pos += self.heading * SPEED * game_info.delta_time.as_secs_f32();

		if self.pos.x < HALF_SIZE
			|| self.pos.y < HALF_SIZE
			|| self.pos.x > FAR_EDGE
			|| self.pos.y > FAR_EDGE
		{
			Action::Remove(vec![self.id])
		} else if let Some(body) = game_info
			.bodies
			.iter()
			.find(|b| b.id > 2 && (self.pos - b.pos).length() < b.radius + HALF_SIZE)
		{
			Action::Remove(vec![self.id, body.id])
		} else {
			Action::Continue()
		}
	}

	fn draw(&self, dt: &mut DrawTarget) {
		let mut pb = PathBuilder::new();
		pb.arc(self.pos.x, self.pos.y, SIZE, 0.0, CIRCLE_ANGLE);
		dt.fill(
			&pb.finish(),
			&Source::Solid(SolidSource::from_unpremultiplied_argb(
				0xff, 0x00, 0x00, 0x00,
			)),
			&DrawOptions::default(),
		);
	}
}

impl GameObject for Bullet {
	fn id(&self) -> usize {
		self.id
	}

	fn body(&self) -> Option<PhysicalBody> {
		None
	}

	fn update(&mut self, game_info: &GameInfo, dt: &mut DrawTarget) -> Result<Action, String> {
		let action = self.update(game_info);
		self.draw(dt);
		Ok(action)
	}
}
