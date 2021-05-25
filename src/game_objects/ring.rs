use std::time::Duration;

use minifb::Window;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};

use super::GameObject;
use crate::GAME_SIZE;

const OFFSET: f32 = GAME_SIZE as f32 / 2.0;
pub const RING_SIZE: f32 = OFFSET / 2.0;
const STROKE_WIDTH: f32 = GAME_SIZE as f32 / 200.0;
const CIRCLE_ANGLE: f32 = std::f32::consts::PI * 2.0;

pub struct Ring;

impl Default for Ring {
	fn default() -> Self {
		Ring
	}
}

impl GameObject for Ring {
	fn update(
		&mut self,
		_: &Window,
		dt: &mut DrawTarget,
		_: &Duration,
		_: &Duration,
	) -> Result<(), String> {
		let mut pb = PathBuilder::new();
		pb.arc(OFFSET, OFFSET, RING_SIZE, 0.0, CIRCLE_ANGLE);
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

		Ok(())
	}
}