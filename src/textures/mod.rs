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
}
