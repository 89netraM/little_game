use std::time::Instant;
use std::time::Duration;

use euclid::{Angle, Transform2D, UnknownUnit, Vector2D};
use minifb::{Key, MouseButton, MouseMode, Window};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use super::{game_object::get_new_object_id, ring::RING_SIZE, Action, Bullet, GameObject};
use crate::GAME_SIZE;

const SPEED: f32 = GAME_SIZE as f32 / 2.0;
const CENTER: Vector2D<f32, UnknownUnit> =
	Vector2D::new(GAME_SIZE as f32 / 2.0, GAME_SIZE as f32 / 2.0);
const SIZE: f32 = GAME_SIZE as f32 / 40.0;
const HALF_SIZE: f32 = SIZE / 2.0;
const QUARTER_SIZE: f32 = HALF_SIZE / 2.0;
const RING_SIZE_SQUARED: f32 = RING_SIZE * RING_SIZE;
const FAR_EDGE: f32 = GAME_SIZE as f32 - HALF_SIZE;
const BULLET_DELAY: f32 = 0.5;

pub struct Player {
	id: usize,
	pos: Vector2D<f32, UnknownUnit>,
	velocity: Vector2D<f32, UnknownUnit>,
	rotation: f32,
	is_shooting: bool,
	previous_bullet: Instant,
}

impl Player {
	fn update(&mut self, window: &Window, delta_time: &Duration) -> Action {
		if (CENTER - self.pos).square_length() < RING_SIZE_SQUARED {
			if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Discard) {
				self.rotation = (mouse_pos.0 - self.pos.x).atan2(mouse_pos.1 - self.pos.y);
			}

			self.velocity = Vector2D::zero();
			if window.is_key_down(Key::A) {
				self.velocity.x -= 1.0;
			}
			if window.is_key_down(Key::D) {
				self.velocity.x += 1.0;
			}
			if window.is_key_down(Key::W) {
				self.velocity.y -= 1.0;
			}
			if window.is_key_down(Key::S) {
				self.velocity.y += 1.0;
			}

			self.is_shooting = window.get_mouse_down(MouseButton::Left);
		}

		if self.velocity != Vector2D::zero() {
			self.pos += self.velocity.normalize() * SPEED * delta_time.as_secs_f32();
		}

		if self.pos.x < HALF_SIZE {
			self.pos.x = HALF_SIZE;
			self.velocity.x = -self.velocity.x;
		}
		if self.pos.y < HALF_SIZE {
			self.pos.y = HALF_SIZE;
			self.velocity.y = -self.velocity.y;
		}
		if self.pos.x > FAR_EDGE {
			self.pos.x = FAR_EDGE;
			self.velocity.x = -self.velocity.x;
		}
		if self.pos.y > FAR_EDGE {
			self.pos.y = FAR_EDGE;
			self.velocity.y = -self.velocity.y;
		}

		if self.is_shooting && self.previous_bullet.elapsed().as_secs_f32() > BULLET_DELAY {
			self.previous_bullet = Instant::now();
			let heading = Vector2D::new(self.rotation.sin(), self.rotation.cos());
			Action::Add(vec![Box::new(Bullet::new(
				self.pos + heading * HALF_SIZE,
				heading,
			))])
		} else {
			Action::Continue()
		}
	}

	fn draw(&self, dt: &mut DrawTarget) {
		let og_transform = *dt.get_transform();
		dt.set_transform(
			&Transform2D::create_rotation(Angle::radians(self.rotation)).post_translate(self.pos),
		);

		dt.fill_rect(
			-HALF_SIZE,
			-HALF_SIZE,
			SIZE,
			SIZE,
			&Source::Solid(SolidSource::from_unpremultiplied_argb(
				0xff, 0xff, 0x00, 0x00,
			)),
			&DrawOptions::default(),
		);
		let mut pb = PathBuilder::new();
		pb.move_to(-QUARTER_SIZE, HALF_SIZE);
		pb.line_to(0.0, QUARTER_SIZE);
		pb.line_to(QUARTER_SIZE, HALF_SIZE);
		dt.fill(
			&pb.finish(),
			&Source::Solid(SolidSource::from_unpremultiplied_argb(
				0xff, 0x00, 0x00, 0x00,
			)),
			&DrawOptions::default(),
		);

		dt.set_transform(&og_transform);
	}
}

impl Default for Player {
	fn default() -> Self {
		Player {
			id: get_new_object_id(),
			pos: CENTER,
			velocity: Vector2D::zero(),
			rotation: 0.0,
			is_shooting: false,
			previous_bullet: Instant::now(),
		}
	}
}

impl GameObject for Player {
	fn id(&self) -> usize {
		self.id
	}

	fn update(
		&mut self,
		window: &Window,
		dt: &mut DrawTarget,
		_: &Duration,
		delta_time: &Duration,
	) -> Result<Action, String> {
		let action = self.update(window, delta_time);
		self.draw(dt);
		Ok(action)
	}
}
