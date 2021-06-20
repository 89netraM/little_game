use instant::Instant;

use super::playing_state::MAZE_CHUNK_SIZE;

const VISIBLE_TIME: f32 = 0.4;

pub struct Monster {
	in_range_time: Option<Instant>,
}

impl Default for Monster {
	fn default() -> Self {
		Self {
			in_range_time: None,
		}
	}
}

impl Monster {
	pub fn update(&mut self, distance: f32) -> bool {
		if let Some(time) = self.in_range_time {
			if distance < MAZE_CHUNK_SIZE {
				time.elapsed().as_secs_f32() < VISIBLE_TIME
			} else {
				self.in_range_time = None;
				true
			}
		} else {
			if distance < MAZE_CHUNK_SIZE {
				self.in_range_time = Some(Instant::now());
			}
			true
		}
	}
}
