use std::f32;

use kiss3d::{
	camera::Camera,
	event::WindowEvent,
	nalgebra::{
		self,
		Isometry3,
		Matrix4,
		Perspective3,
		Point3,
		Unit,
		UnitQuaternion,
		Vector2,
		Vector3,
	},
	resource::ShaderUniform,
	window::Canvas,
};

const LOOK_STEP: f32 = 0.0025;
const MOVE_STEP: f32 = 0.05;

pub struct FirstPerson {
	eye: Point3<f32>,
	yaw: f32,
	pitch: f32,
	projection: Perspective3<f32>,
	proj: Matrix4<f32>,
	view: Matrix4<f32>,
	proj_view: Matrix4<f32>,
	inverse_proj_view: Matrix4<f32>,
	coord_system: CoordSystemRh,
}

impl FirstPerson {
	pub fn new(eye: Point3<f32>, at: Point3<f32>) -> FirstPerson {
		FirstPerson::new_with_frustrum(f32::consts::PI / 4.0, 0.05, 1024.0, eye, at)
	}

	pub fn new_with_frustrum(
		fov: f32,
		znear: f32,
		zfar: f32,
		eye: Point3<f32>,
		at: Point3<f32>,
	) -> FirstPerson {
		let mut res = FirstPerson {
			eye: Point3::new(0.0, 0.0, 0.0),
			yaw: 0.0,
			pitch: 0.0,
			projection: Perspective3::new(800.0 / 600.0, fov, znear, zfar),
			proj: nalgebra::zero(),
			view: nalgebra::zero(),
			proj_view: nalgebra::zero(),
			inverse_proj_view: nalgebra::zero(),
			coord_system: CoordSystemRh::from_up_axis(Vector3::y_axis()),
		};

		res.look_at(eye, at);

		res
	}

	fn look_at(&mut self, eye: Point3<f32>, at: Point3<f32>) {
		let dist = (eye - at).norm();

		let view_eye = self.coord_system.rotation_to_y_up * eye;
		let view_at = self.coord_system.rotation_to_y_up * at;
		let pitch = ((view_at.y - view_eye.y) / dist).acos();
		let yaw = (view_at.z - view_eye.z).atan2(view_at.x - view_eye.x);

		self.eye = eye;
		self.yaw = yaw;
		self.pitch = pitch;
		self.update_projviews();
	}

	fn at(&self) -> Point3<f32> {
		let view_eye = self.coord_system.rotation_to_y_up * self.eye;
		let ax = view_eye.x + self.yaw.cos() * self.pitch.sin();
		let ay = view_eye.y + self.pitch.cos();
		let az = view_eye.z + self.yaw.sin() * self.pitch.sin();
		self.coord_system.rotation_to_y_up.inverse() * Point3::new(ax, ay, az)
	}

	fn update_restrictions(&mut self) {
		if self.pitch <= 0.01 {
			self.pitch = 0.01
		}

		let _pi: f32 = f32::consts::PI;
		if self.pitch > _pi - 0.01 {
			self.pitch = _pi - 0.01
		}
	}

	fn handle_left_button_displacement(&mut self, dpos: &Vector2<f32>) {
		self.yaw += dpos.x * LOOK_STEP;
		self.pitch += dpos.y * LOOK_STEP;

		self.update_restrictions();
		self.update_projviews();
	}

	fn update_projviews(&mut self) {
		self.view = self.view_transform().to_homogeneous();
		self.proj = *self.projection.as_matrix();
		self.proj_view = self.proj * self.view;
		let _ = self
			.proj_view
			.try_inverse()
			.map(|inverse_proj| self.inverse_proj_view = inverse_proj);
	}

	pub fn move_dir(&self, up: bool, down: bool, right: bool, left: bool) -> Option<Vector3<f32>> {
		let t = self.observer_frame();
		let front_v = t * Vector3::z();
		let right_v = t * Vector3::x();

		let mut movement = nalgebra::zero::<Vector3<f32>>();
		if up || down || right || left {
			if up {
				movement += front_v;
			}
			if down {
				movement -= front_v;
			}
			if right {
				movement -= right_v;
			}
			if left {
				movement += right_v;
			}
			movement.y = 0.0;
			movement.set_magnitude(MOVE_STEP);
			Some(movement)
		} else {
			None
		}
	}

	#[inline]
	pub fn eye(&self) -> &Point3<f32> {
		&self.eye
	}

	#[inline]
	pub fn set_eye(&mut self, eye: Point3<f32>) {
		self.eye = eye;
		self.update_restrictions();
		self.update_projviews();
	}

	fn observer_frame(&self) -> Isometry3<f32> {
		Isometry3::face_towards(&self.eye, &self.at(), &self.coord_system.up_axis)
	}
}

impl Camera for FirstPerson {
	fn clip_planes(&self) -> (f32, f32) {
		(self.projection.znear(), self.projection.zfar())
	}

	fn view_transform(&self) -> Isometry3<f32> {
		Isometry3::look_at_rh(&self.eye, &self.at(), &self.coord_system.up_axis)
	}

	fn handle_event(&mut self, canvas: &Canvas, event: &WindowEvent) {
		match *event {
			WindowEvent::CursorPos(x, y, _) => {
				let curr_pos = Vector2::new(x as f32, y as f32);

				let size = canvas.size();
				let center = Vector2::new(size.0 as f32 / 2.0, size.1 as f32 / 2.0);
				let dpos = curr_pos - center;
				self.handle_left_button_displacement(&dpos);
				canvas.set_cursor_position(center.x as f64, center.y as f64);
			}
			WindowEvent::FramebufferSize(w, h) => {
				self.projection.set_aspect(w as f32 / h as f32);
				self.update_projviews();
			}
			_ => {}
		}
	}

	fn eye(&self) -> Point3<f32> {
		self.eye
	}

	fn transformation(&self) -> Matrix4<f32> {
		self.proj_view
	}

	fn inverse_transformation(&self) -> Matrix4<f32> {
		self.inverse_proj_view
	}

	#[inline]
	fn upload(
		&self,
		_: usize,
		proj: &mut ShaderUniform<Matrix4<f32>>,
		view: &mut ShaderUniform<Matrix4<f32>>,
	) {
		proj.upload(&self.proj);
		view.upload(&self.view);
	}

	fn update(&mut self, _: &Canvas) {
	}
}

struct CoordSystemRh {
	up_axis: Unit<Vector3<f32>>,
	rotation_to_y_up: UnitQuaternion<f32>,
}

impl CoordSystemRh {
	#[inline]
	fn from_up_axis(up_axis: Unit<Vector3<f32>>) -> Self {
		let rotation_to_y_up = UnitQuaternion::rotation_between_axis(&up_axis, &Vector3::y_axis())
			.unwrap_or_else(|| {
				UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::PI)
			});
		Self {
			up_axis,
			rotation_to_y_up,
		}
	}
}
