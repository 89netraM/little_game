mod materials;

use image::{DynamicImage, RgbImage};
use kiss3d::resource::TextureManager;

use self::materials::init_materials;

const ONE: &[u8] = &[0x1C, 0x0F, 0x0A];
const TWO: &[u8] = &[0x29, 0x13, 0x0A];
const THREE: &[u8] = &[0x24, 0x1E, 0x0F];
// const FOUR: &[u8] = &[0x26, 0x1D, 0x18];
// const FIVE: &[u8] = &[0xD6, 0x92, 0x65];

pub fn init_textures() {
	TextureManager::get_global_manager(add_textures);
	init_materials();
}

fn add_textures(manager: &mut TextureManager) {
	let mut wall = Vec::with_capacity(14 * 16);
	for _ in 0..14 {
		wall.extend_from_slice(&ONE);
	}
	for _ in 0..14 {
		wall.extend_from_slice(&ONE);
		for _ in 0..6 {
			wall.extend_from_slice(&THREE);
			wall.extend_from_slice(&TWO);
		}
		wall.extend_from_slice(&ONE);
	}
	for _ in 0..14 {
		wall.extend_from_slice(&ONE);
	}
	manager.add_image(
		DynamicImage::ImageRgb8(RgbImage::from_raw(14, 16, wall).unwrap()),
		"wall",
	);

	let mut floor = Vec::with_capacity(14 * 14);
	for _ in 0..7 {
		floor.extend_from_slice(&ONE);
		floor.extend_from_slice(&TWO);
	}
	for _ in 0..6 {
		for _ in 0..13 {
			floor.extend_from_slice(&TWO);
		}
		floor.extend_from_slice(&ONE);
		floor.extend_from_slice(&ONE);
		for _ in 0..13 {
			floor.extend_from_slice(&TWO);
		}
	}
	for _ in 0..7 {
		floor.extend_from_slice(&TWO);
		floor.extend_from_slice(&ONE);
	}
	manager.add_image(
		DynamicImage::ImageRgb8(RgbImage::from_raw(14, 14, floor).unwrap()),
		"floor",
	);
}
