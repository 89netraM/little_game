mod materials;

use kiss3d::resource::TextureManager;

use self::materials::init_materials;

pub fn init_textures() {
	TextureManager::get_global_manager(add_textures);
	init_materials();
}

fn add_textures(manager: &mut TextureManager) {
	manager.add_image_from_memory(include_bytes!("./wall.png"), "wall");
	manager.add_image_from_memory(include_bytes!("./ceiling.png"), "ceiling");
	manager.add_image_from_memory(include_bytes!("./floor.png"), "floor");
	manager.add_image_from_memory(include_bytes!("./monster.png"), "monster");
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
	let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
	let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
	let m = l - c / 2.0;

	let (r_temp, g_temp, b_temp) = if h <= 1.0 / 6.0 {
		(c, x, 0.0)
	} else if h <= 2.0 / 6.0 {
		(x, c, 0.0)
	} else if h <= 3.0 / 6.0 {
		(0.0, c, x)
	} else if h <= 4.0 / 6.0 {
		(0.0, x, c)
	} else if h <= 5.0 / 6.0 {
		(x, 0.0, c)
	} else {
		(c, 0.0, x)
	};

	(r_temp + m, g_temp + m, b_temp + m)
}
