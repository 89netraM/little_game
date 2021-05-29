use kiss3d::nalgebra::Point3;

use super::MAZE_SIZE_HALF;

const WALL_THICKNESS: f32 = 0.1;

pub enum Wall {
	Horizontal(Point3<f32>),
	Vertical(Point3<f32>),
}

impl Wall {
	pub fn push_back(&self, point: &mut Point3<f32>) {
		match self {
			Wall::Horizontal(p) => {
				if p.z - MAZE_SIZE_HALF <= point.z && point.z <= p.z + MAZE_SIZE_HALF {
					if p.x - WALL_THICKNESS < point.x && point.x <= p.x {
						point.x = p.x - WALL_THICKNESS;
					} else if p.x <= point.x && point.x <= p.x + WALL_THICKNESS {
						point.x = p.x + WALL_THICKNESS;
					}
				}
			}
			Wall::Vertical(p) => {
				if p.x - MAZE_SIZE_HALF <= point.x && point.x <= p.x + MAZE_SIZE_HALF {
					if p.z - WALL_THICKNESS < point.z && point.z <= p.z {
						point.z = p.z - WALL_THICKNESS;
					} else if p.z <= point.z && point.z <= p.z + WALL_THICKNESS {
						point.z = p.z + WALL_THICKNESS;
					}
				}
			}
		}
	}
}
