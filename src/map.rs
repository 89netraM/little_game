use std::{collections::HashSet, iter::FromIterator};

use rand::{rngs::StdRng, Rng};

use super::rng::rng_for_maze;

pub const ROOM_SIZE: usize = 5;
pub const ROOM_CENTER: usize = ROOM_SIZE / 2;
pub const MAP_LENGTH: usize = ROOM_SIZE * 2 * ROOM_SIZE - (ROOM_SIZE + ROOM_SIZE);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	Up,
	Left,
	Right,
	Down,
}

const DIRECTIONS: [Direction; 4] = [
	Direction::Up,
	Direction::Left,
	Direction::Right,
	Direction::Down,
];

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct Position(
	/// Row
	pub usize,
	/// Column
	pub usize,
);

pub struct Map([bool; MAP_LENGTH]);

impl Map {
	fn new() -> Map {
		Map([true; MAP_LENGTH])
	}

	pub fn generate_prim(seed: u64, position: (i64, i64)) -> Map {
		let mut map = Map::new();
		let mut rng: StdRng = rng_for_maze(seed, position);

		let mut visited = HashSet::new();
		visited.insert(Position(ROOM_CENTER, ROOM_CENTER));
		let mut walls: Vec<_> = map.walls_around(&Position(ROOM_CENTER, ROOM_CENTER));

		while !walls.is_empty() {
			let index = rng.gen_range(0..walls.len());
			let (from, dir) = walls.remove(index);
			if let Some(to) = map.move_in_direction(&from, &dir) {
				if !visited.contains(&to) {
					map.set(&from, &dir, false);

					visited.insert(to);
					walls.extend(map.walls_around::<Vec<_>>(&to));
				}
			}
		}

		map
	}

	#[inline]
	pub fn set_above(&mut self, pos: &Position, closed: bool) {
		self.set_below(&Position(pos.0 - 1, pos.1), closed);
	}

	#[inline]
	pub fn is_above(&self, pos: &Position) -> bool {
		self.is_below(&Position(pos.0 - 1, pos.1))
	}

	#[inline]
	pub fn set_left(&mut self, pos: &Position, closed: bool) {
		self.set_right(&Position(pos.0, pos.1 - 1), closed);
	}

	#[inline]
	pub fn is_left(&self, pos: &Position) -> bool {
		self.is_right(&Position(pos.0, pos.1 - 1))
	}

	#[inline]
	pub fn set_right(&mut self, pos: &Position, closed: bool) {
		debug_assert!(pos.0 < ROOM_SIZE && pos.1 < ROOM_SIZE - 1);

		self.0[(ROOM_SIZE - 1) * ROOM_SIZE + pos.0 * (ROOM_SIZE - 1) + pos.1] = closed;
	}

	#[inline]
	pub fn is_right(&self, pos: &Position) -> bool {
		debug_assert!(pos.0 < ROOM_SIZE && pos.1 < ROOM_SIZE - 1);

		self.0[(ROOM_SIZE - 1) * ROOM_SIZE + pos.0 * (ROOM_SIZE - 1) + pos.1]
	}

	#[inline]
	pub fn set_below(&mut self, pos: &Position, closed: bool) {
		debug_assert!(pos.0 < ROOM_SIZE - 1 && pos.1 < ROOM_SIZE);

		self.0[pos.0 * ROOM_SIZE + pos.1] = closed;
	}

	#[inline]
	pub fn is_below(&self, pos: &Position) -> bool {
		debug_assert!(pos.0 < ROOM_SIZE - 1 && pos.1 < ROOM_SIZE);

		self.0[pos.0 * ROOM_SIZE + pos.1]
	}

	pub fn set(&mut self, pos: &Position, dir: &Direction, closed: bool) {
		match dir {
			Direction::Up => self.set_above(pos, closed),
			Direction::Left => self.set_left(pos, closed),
			Direction::Right => self.set_right(pos, closed),
			Direction::Down => self.set_below(pos, closed),
		};
	}

	pub fn is(&self, pos: &Position, dir: &Direction) -> Option<bool> {
		match dir {
			Direction::Up if 0 < pos.0 && pos.0 < ROOM_SIZE && pos.1 < ROOM_SIZE => {
				Some(self.is_above(pos))
			}
			Direction::Left if pos.0 < ROOM_SIZE && 0 < pos.1 && pos.1 < ROOM_SIZE => {
				Some(self.is_left(pos))
			}
			Direction::Right if pos.0 < ROOM_SIZE && pos.1 < ROOM_SIZE - 1 => {
				Some(self.is_right(pos))
			}
			Direction::Down if pos.0 < ROOM_SIZE - 1 && pos.1 < ROOM_SIZE => {
				Some(self.is_below(pos))
			}
			_ => None,
		}
	}

	fn move_in_direction(&self, current: &Position, dir: &Direction) -> Option<Position> {
		match dir {
			Direction::Up if current.0 > 0 => Some(Position(current.0 - 1, current.1)),
			Direction::Left if current.1 > 0 => Some(Position(current.0, current.1 - 1)),
			Direction::Right if current.1 < ROOM_SIZE - 1 => {
				Some(Position(current.0, current.1 + 1))
			}
			Direction::Down if current.0 < ROOM_SIZE - 1 => {
				Some(Position(current.0 + 1, current.1))
			}
			_ => None,
		}
	}

	fn walls_around<V>(&self, pos: &Position) -> V
	where
		V: FromIterator<(Position, Direction)>,
	{
		DIRECTIONS
			.iter()
			.filter_map(|dir| {
				if self.is(pos, dir) == Some(true) {
					return Some((*pos, *dir));
				}
				None
			})
			.collect()
	}
}
