use rand::{Rng, SeedableRng};

use super::map::Direction;

pub fn rng_for_maze<R: SeedableRng>(seed: u64, position: (i64, i64)) -> R {
	rng_from_bytes(&[
		&seed.to_be_bytes(),
		&position.0.to_be_bytes(),
		&position.1.to_be_bytes(),
	])
}

const DOOR_ODDS: f32 = 0.8;
pub fn rand_for_border_walls<R: SeedableRng + Rng>(
	seed: u64,
	mut position: (i64, i64),
	mut direction: Direction,
	max: usize,
) -> Option<usize> {
	match direction {
		Direction::Up if position.1 > 0 => {
			direction = Direction::Down;
			position.1 -= 1;
		}
		Direction::Left if position.0 > 0 => {
			direction = Direction::Right;
			position.0 -= 1;
		}
		Direction::Right if position.0 < 0 => {
			direction = Direction::Left;
			position.0 += 1;
		}
		Direction::Down if position.1 < 0 => {
			direction = Direction::Up;
			position.1 += 1;
		}
		_ => {}
	};

	let mut rng: R = rng_from_bytes(&[
		&seed.to_be_bytes(),
		&position.0.to_be_bytes(),
		&position.1.to_be_bytes(),
		&[direction as u8],
	]);
	if rng.gen::<f32>() < DOOR_ODDS {
		Some(rng.gen_range(0..max))
	} else {
		None
	}
}

const ITEM_CHUNK_SIZE: i64 = 5;
pub fn rng_for_item<R: SeedableRng + Rng>(seed: u64, position: (i64, i64)) -> Positions<R> {
	let r = position.0 / ITEM_CHUNK_SIZE;
	let c = position.1 / ITEM_CHUNK_SIZE;
	Positions {
		rng: rng_from_bytes(&[&seed.to_be_bytes(), &r.to_be_bytes(), &c.to_be_bytes()]),
		open_positions: (r * ITEM_CHUNK_SIZE..(r + 1) * ITEM_CHUNK_SIZE)
			.flat_map(|r| (c * ITEM_CHUNK_SIZE..(c + 1) * ITEM_CHUNK_SIZE).map(move |c| (r, c)))
			.collect(),
	}
}

pub struct Positions<R> {
	rng: R,
	open_positions: Vec<(i64, i64)>,
}

impl<R: Rng> Iterator for Positions<R> {
	type Item = (i64, i64);

	fn next(&mut self) -> Option<Self::Item> {
		if self.open_positions.is_empty() {
			None
		} else {
			Some(
				self.open_positions
					.swap_remove(self.rng.gen_range(0..self.open_positions.len())),
			)
		}
	}
}

fn rng_from_bytes<R: SeedableRng>(seeds: &[&[u8]]) -> R {
	let mut seed = <R as SeedableRng>::Seed::default();
	let seed_slice = seed.as_mut();
	let mut seed_index = 0;
	let seed_length = seed_slice.len();
	for slice in seeds {
		for byte in *slice {
			seed_slice[seed_index % seed_length] ^= byte;
			seed_index += 1;
		}
	}
	R::from_seed(seed)
}

#[cfg(test)]
mod tests {
	use rand::rngs::StdRng;

	use super::{super::map::Direction, *};

	#[test]
	fn upward_stability() {
		let up_origin = rand_for_border_walls::<StdRng>(0, (0, 0), Direction::Up, 5);
		let down_above_origin = rand_for_border_walls::<StdRng>(0, (0, -1), Direction::Down, 5);
		assert_eq!(up_origin, down_above_origin);
	}

	#[test]
	fn left_stability() {
		let left_origin = rand_for_border_walls::<StdRng>(0, (0, 0), Direction::Left, 5);
		let right_left_origin = rand_for_border_walls::<StdRng>(0, (-1, 0), Direction::Right, 5);
		assert_eq!(left_origin, right_left_origin);
	}

	#[test]
	fn right_stability() {
		let right_origin = rand_for_border_walls::<StdRng>(0, (0, 0), Direction::Right, 5);
		let left_right_origin = rand_for_border_walls::<StdRng>(0, (1, 0), Direction::Left, 5);
		assert_eq!(right_origin, left_right_origin);
	}

	#[test]
	fn downward_stability() {
		let down_origin = rand_for_border_walls::<StdRng>(0, (0, 0), Direction::Down, 5);
		let up_below_origin = rand_for_border_walls::<StdRng>(0, (0, 1), Direction::Up, 5);
		assert_eq!(down_origin, up_below_origin);
	}
}
