mod camera;

use kiss3d::{
	light::Light,
	nalgebra::{Point3, Translation3, UnitQuaternion, Vector3},
	window::Window,
};

use self::camera::FirstPerson;

fn main() {
	let mut window = Window::new_with_size("Lazer aMAZEing", 1280, 800);
	window.hide_cursor(true);
	window.set_cursor_grab(true);
	window.set_light(Light::StickToCamera);

	let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, -1.0));

	let mut cube1 = window.add_cube(0.5, 0.5, 0.5);
	cube1.set_color(0.0, 1.0, 1.0);
	cube1.append_translation(&Translation3::new(0.0, 0.0, 2.0));
	let mut cube2 = window.add_cube(0.5, 0.5, 0.5);
	cube2.set_color(1.0, 0.0, 1.0);
	cube2.append_translation(&Translation3::new(2.0, 0.0, 0.0));
	let mut cube3 = window.add_cube(0.5, 0.5, 0.5);
	cube3.set_color(1.0, 1.0, 0.0);
	cube3.append_translation(&Translation3::new(0.0, 0.0, -2.0));
	let mut cube4 = window.add_cube(0.5, 0.5, 0.5);
	cube4.set_color(0.0, 1.0, 0.0);
	cube4.append_translation(&Translation3::new(-2.0, 0.0, 0.0));

	let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

	while window.render_with_camera(&mut camera) {
		cube1.prepend_to_local_rotation(&rotation);
		cube2.prepend_to_local_rotation(&rotation);
		cube3.prepend_to_local_rotation(&rotation);
		cube4.prepend_to_local_rotation(&rotation);
	}
}
