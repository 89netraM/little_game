use rand::{Rng, SeedableRng};

use super::map::Direction;

pub fn rng_for_maze<R: SeedableRng>(seed: u64, position: (i64, i64)) -> R {
	rng_from_bytes(&[
		&seed.to_be_bytes(),
		&position.0.to_be_bytes(),
		&position.1.to_be_bytes(),
	])
}

pub fn rand_for_border_walls<R: SeedableRng + Rng>(
	seed: u64,
	mut position: (i64, i64),
	mut direction: Direction,
	max: usize,
) -> usize {
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
	rng.gen_range(0..max)
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
