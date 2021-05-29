use kiss3d::{
	camera::FirstPerson,
	light::Light,
	nalgebra::{Point3, UnitQuaternion, Vector3},
	window::Window,
};

fn main() {
	let mut window = Window::new_with_size("Lazer aMAZEing", 1280, 800);
	window.set_light(Light::Absolute(Point3::new(0.0, 2.0, 2.0)));

	let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, -1.0), Point3::new(0.0, 0.0, 0.0));

	let mut cube = window.add_cube(1.0, 1.0, 1.0);
	cube.set_color(0.0, 1.0, 1.0);
	let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

	while window.render_with_camera(&mut camera) {
		cube.prepend_to_local_rotation(&rotation);
	}
}
