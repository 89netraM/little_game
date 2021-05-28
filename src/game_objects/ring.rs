use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};

use super::{game_object::get_new_object_id, Action, GameObject, PhysicalBody};
use crate::{game_info::GameInfo, GAME_SIZE};

const OFFSET: f32 = GAME_SIZE as f32 / 2.0;
const RING_SIZE: f32 = OFFSET / 2.0;
const RING_SIZE_DECREASE: f32 = RING_SIZE * 0.01;
const STROKE_WIDTH: f32 = GAME_SIZE as f32 / 200.0;
const CIRCLE_ANGLE: f32 = std::f32::consts::PI * 2.0;

pub struct Ring {
	id: usize,
	radius: f32,
}

impl Ring {
	pub fn shrink(&mut self) {
		self.radius -= RING_SIZE_DECREASE;
	}

	pub fn radius(&self) -> f32 {
		self.radius
	}
}

impl Default for Ring {
	fn default() -> Self {
		Ring {
			id: get_new_object_id(),
			radius: RING_SIZE,
		}
	}
}

impl GameObject for Ring {
	fn id(&self) -> usize {
		self.id
	}

	fn body(&self) -> Option<PhysicalBody> {
		None
	}

	fn update(&mut self, _: &GameInfo, dt: &mut DrawTarget) -> Result<Vec<Action>, String> {
		let mut pb = PathBuilder::new();
		pb.arc(OFFSET, OFFSET, self.radius, 0.0, CIRCLE_ANGLE);
		dt.stroke(
			&pb.finish(),
			&Source::Solid(SolidSource::from_unpremultiplied_argb(
				0xff, 0x00, 0xff, 0xff,
			)),
			&StrokeStyle {
				width: STROKE_WIDTH,
				..StrokeStyle::default()
			},
			&DrawOptions::default(),
		);

		Ok(Vec::new())
	}
}
