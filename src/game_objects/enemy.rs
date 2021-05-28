use std::time::{Duration, Instant};

use euclid::{Angle, Transform2D, UnknownUnit, Vector2D};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use super::{game_object::get_new_object_id, ring::RING_SIZE, Action, GameObject, PhysicalBody};
use crate::{game_info::GameInfo, GAME_SIZE};

const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
const PI2: f32 = std::f32::consts::PI * 2.0;
const GAME_SIZE_F32: f32 = GAME_SIZE as f32;
const HALF_GAME_SIZE: f32 = GAME_SIZE_F32 as f32 / 2.0;
const CENTER: Vector2D<f32, UnknownUnit> = Vector2D::new(HALF_GAME_SIZE, HALF_GAME_SIZE);
const EDGE: Vector2D<f32, UnknownUnit> = Vector2D::new(GAME_SIZE_F32, GAME_SIZE_F32);

const SPEED: f32 = GAME_SIZE as f32 / 4.0;
const SIZE: f32 = GAME_SIZE as f32 / 40.0;
const HALF_SIZE: f32 = SIZE / 2.0;
const QUARTER_SIZE: f32 = HALF_SIZE / 2.0;
const RING_SIZE_SQUARED: f32 = RING_SIZE * RING_SIZE;

pub struct EnemySpawner {
	id: usize,
	last_enemy_spawn: Instant,
}

impl Default for EnemySpawner {
	fn default() -> Self {
		EnemySpawner {
			id: get_new_object_id(),
			last_enemy_spawn: Instant::now(),
		}
	}
}

impl GameObject for EnemySpawner {
	fn id(&self) -> usize {
		self.id
	}

	fn body(&self) -> Option<PhysicalBody> {
		None
	}

	fn update(&mut self, _: &GameInfo, _: &mut DrawTarget) -> Result<Vec<Action>, String> {
		if self.last_enemy_spawn.elapsed().as_secs_f32() > ENEMY_SPAWN_INTERVAL {
			self.last_enemy_spawn = Instant::now();
			let rotation = rand::random::<f32>() * PI2;
			let heading = Vector2D::new(rotation.sin(), rotation.cos());
			Ok(vec![Action::Add(Box::new(Enemy::new(
				(CENTER - heading * HALF_GAME_SIZE).clamp(Vector2D::zero(), EDGE),
				heading,
			)))])
		} else {
			Ok(Vec::new())
		}
	}
}

pub struct Enemy {
	id: usize,
	pos: Vector2D<f32, UnknownUnit>,
	heading: Vector2D<f32, UnknownUnit>,
}

impl Enemy {
	pub fn new(pos: Vector2D<f32, UnknownUnit>, heading: Vector2D<f32, UnknownUnit>) -> Self {
		Enemy {
			id: get_new_object_id(),
			pos,
			heading,
		}
	}

	fn update(&mut self, delta_time: &Duration) -> Vec<Action> {
		if (CENTER - self.pos).square_length() < RING_SIZE_SQUARED {
			vec![Action::Remove(self.id())]
		} else {
			self.pos += self.heading * SPEED * delta_time.as_secs_f32();
			Vec::new()
		}
	}

	fn draw(&self, dt: &mut DrawTarget) {
		let og_transform = *dt.get_transform();
		dt.set_transform(
			&Transform2D::create_rotation(Angle::radians(self.heading.x.atan2(self.heading.y)))
				.post_translate(self.pos),
		);

		dt.fill_rect(
			-HALF_SIZE,
			-HALF_SIZE,
			SIZE,
			SIZE,
			&Source::Solid(SolidSource::from_unpremultiplied_argb(
				0xff, 0x00, 0x00, 0xff,
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

impl GameObject for Enemy {
	fn id(&self) -> usize {
		self.id
	}

	fn body(&self) -> Option<PhysicalBody> {
		Some(PhysicalBody {
			id: self.id,
			pos: self.pos,
			radius: HALF_SIZE,
		})
	}

	fn update(&mut self, game_info: &GameInfo, dt: &mut DrawTarget) -> Result<Vec<Action>, String> {
		let action = self.update(&game_info.delta_time);
		self.draw(dt);
		Ok(action)
	}
}
