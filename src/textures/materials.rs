use std::{cell::RefCell, rc::Rc};

use kiss3d::{
	camera::Camera,
	context::Context,
	light::Light,
	nalgebra::{Isometry3, Matrix3, Matrix4, Point2, Point3, Vector3},
	resource::{Effect, Material, MaterialManager, Mesh, ShaderAttribute, ShaderUniform},
	scene::ObjectData,
};

pub fn init_materials() {
	MaterialManager::get_global_manager(add_materials);
}

fn add_materials(manager: &mut MaterialManager) {
	let pm = Rc::new(RefCell::new(
		Box::new(PixelMaterial::new()) as Box<dyn Material + 'static>
	));
	manager.add(pm, "pixel");
}

struct PixelMaterial {
	pos: ShaderAttribute<Point3<f32>>,
	normal: ShaderAttribute<Vector3<f32>>,
	tex_coord: ShaderAttribute<Point2<f32>>,
	color: ShaderUniform<Point3<f32>>,
	transform: ShaderUniform<Matrix4<f32>>,
	scale: ShaderUniform<Matrix3<f32>>,
	n_transform: ShaderUniform<Matrix3<f32>>,
	view: ShaderUniform<Matrix4<f32>>,
	proj: ShaderUniform<Matrix4<f32>>,
	effect: Effect,
}

impl PixelMaterial {
	pub fn new() -> Self {
		let effect = Effect::new_from_str(OBJECT_VERTEX_SRC, OBJECT_FRAGMENT_SRC);
		PixelMaterial {
			pos: effect.get_attrib("position").unwrap(),
			normal: effect.get_attrib("normal").unwrap(),
			tex_coord: effect.get_attrib("tex_coord").unwrap(),
			color: effect.get_uniform("color").unwrap(),
			transform: effect.get_uniform("transform").unwrap(),
			scale: effect.get_uniform("scale").unwrap(),
			n_transform: effect.get_uniform("n_transform").unwrap(),
			view: effect.get_uniform("view").unwrap(),
			proj: effect.get_uniform("proj").unwrap(),
			effect,
		}
	}

	fn activate(&mut self) {
		self.effect.use_program();
		self.pos.enable();
		self.normal.enable();
		self.tex_coord.enable();
	}

	fn deactivate(&mut self) {
		self.tex_coord.disable();
		self.normal.disable();
		self.pos.disable();
	}
}

impl Material for PixelMaterial {
	fn render(
		&mut self,
		pass: usize,
		transform: &Isometry3<f32>,
		scale: &Vector3<f32>,
		camera: &mut dyn Camera,
		_: &Light,
		data: &ObjectData,
		mesh: &mut Mesh,
	) {
		let ctx = Context::get();
		self.activate();

		camera.upload(pass, &mut self.proj, &mut self.view);

		let formatted_transform = transform.to_homogeneous();
		let formatted_n_transform = transform.rotation.to_rotation_matrix().into_inner();
		let formatted_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

		self.transform.upload(&formatted_transform);
		self.n_transform.upload(&formatted_n_transform);
		self.scale.upload(&formatted_scale);

		mesh.bind(&mut self.pos, &mut self.normal, &mut self.tex_coord);

		ctx.active_texture(Context::TEXTURE0);
		ctx.bind_texture(Context::TEXTURE_2D, Some(data.texture()));
		ctx.tex_parameteri(
			Context::TEXTURE_2D,
			Context::TEXTURE_MIN_FILTER,
			Context::NEAREST as i32,
		);
		ctx.tex_parameteri(
			Context::TEXTURE_2D,
			Context::TEXTURE_MAG_FILTER,
			Context::NEAREST as i32,
		);

		if data.surface_rendering_active() {
			self.color.upload(data.color());

			let _ = ctx.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
			ctx.draw_elements(
				Context::TRIANGLES,
				mesh.num_pts() as i32,
				Context::UNSIGNED_SHORT,
				0,
			);
		}

		mesh.unbind();
		self.deactivate();
	}
}

const OBJECT_VERTEX_SRC: &str = include_str!("./pixel.vert");
const OBJECT_FRAGMENT_SRC: &str = include_str!("./pixel.frag");
