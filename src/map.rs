use std::{collections::HashSet, iter::FromIterator};

use rand::{thread_rng, Rng};

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

pub struct Map {
	pub rows: usize,
	pub columns: usize,
	map: Box<[bool]>,
}

impl Map {
	fn new(rows: usize, columns: usize) -> Map {
		Map {
			rows,
			columns,
			map: vec![true; rows * 2 * columns - (rows + columns)].into_boxed_slice(),
		}
	}

	pub fn generate_prim(rows: usize, columns: usize, start: Position) -> Map {
		let mut map = Map::new(rows, columns);
		let mut rng = thread_rng();

		let mut visited = HashSet::new();
		visited.insert(start);
		let mut walls: Vec<_> = map.walls_around(&start);

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
		assert!(pos.0 < self.rows && pos.1 < self.columns - 1);

		self.map[(self.rows - 1) * self.columns + pos.0 * (self.columns - 1) + pos.1] = closed;
	}

	#[inline]
	pub fn is_right(&self, pos: &Position) -> bool {
		assert!(pos.0 < self.rows && pos.1 < self.columns - 1);

		self.map[(self.rows - 1) * self.columns + pos.0 * (self.columns - 1) + pos.1]
	}

	#[inline]
	pub fn set_below(&mut self, pos: &Position, closed: bool) {
		assert!(pos.0 < self.rows - 1 && pos.1 < self.columns);

		self.map[pos.0 * self.columns + pos.1] = closed;
	}

	#[inline]
	pub fn is_below(&self, pos: &Position) -> bool {
		assert!(pos.0 < self.rows - 1 && pos.1 < self.columns);

		self.map[pos.0 * self.columns + pos.1]
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
			Direction::Up if 0 < pos.0 && pos.0 < self.rows && pos.1 < self.columns => {
				Some(self.is_above(pos))
			}
			Direction::Left if pos.0 < self.rows && 0 < pos.1 && pos.1 < self.columns => {
				Some(self.is_left(pos))
			}
			Direction::Right if pos.0 < self.rows && pos.1 < self.columns - 1 => {
				Some(self.is_right(pos))
			}
			Direction::Down if pos.0 < self.rows - 1 && pos.1 < self.columns => {
				Some(self.is_below(pos))
			}
			_ => None,
		}
	}

	fn move_in_direction(&self, current: &Position, dir: &Direction) -> Option<Position> {
		match dir {
			Direction::Up if current.0 > 0 => Some(Position(current.0 - 1, current.1)),
			Direction::Left if current.1 > 0 => Some(Position(current.0, current.1 - 1)),
			Direction::Right if current.1 < self.columns - 1 => {
				Some(Position(current.0, current.1 + 1))
			}
			Direction::Down if current.0 < self.rows - 1 => {
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
