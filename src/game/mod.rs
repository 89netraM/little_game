use kiss3d::{
	camera::Camera,
	planar_camera::PlanarCamera,
	post_processing::PostProcessingEffect,
	renderer::Renderer,
	window::{State, Window},
};

mod menu_state;
mod playing_state;
mod wall;

pub use menu_state::MenuState;
pub use playing_state::PlayingState;

pub type CamerasEffectRenderer<'a> = (
	Option<&'a mut dyn Camera>,
	Option<&'a mut dyn PlanarCamera>,
	Option<&'a mut dyn Renderer>,
	Option<&'a mut dyn PostProcessingEffect>,
);

pub trait InnerGameState {
	fn init(&mut self, _window: &mut Window) {
	}
	fn step(&mut self, window: &mut Window) -> Option<Box<dyn InnerGameState>>;
	fn cameras_and_effect_and_renderer(&mut self) -> CamerasEffectRenderer {
		(None, None, None, None)
	}
	fn clean(&mut self, _window: &mut Window) {
	}
}

pub struct GameState(Box<dyn InnerGameState>);

impl GameState {
	pub fn new(inner_state: Box<dyn InnerGameState>) -> Self {
		Self(inner_state)
	}
}

impl State for GameState {
	fn step(&mut self, window: &mut Window) {
		while let Some(new_state) = self.0.step(window) {
			self.0.clean(window);
			self.0 = new_state;
			self.0.init(window);
		}
	}

	fn cameras_and_effect_and_renderer(&mut self) -> CamerasEffectRenderer {
		self.0.cameras_and_effect_and_renderer()
	}
}
