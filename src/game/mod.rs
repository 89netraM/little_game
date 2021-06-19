use kiss3d::{
	camera::Camera,
	event::{Action, Key, MouseButton, WindowEvent},
	planar_camera::PlanarCamera,
	post_processing::PostProcessingEffect,
	renderer::Renderer,
	window::{State, Window},
};

mod menu_state;
mod pause_state;
mod playing_state;
mod wall;

pub use menu_state::MenuState;
pub use pause_state::PauseState;
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
	fn step(
		&mut self,
		window: &mut Window,
		mouse_buttons: &MouseButtons,
	) -> Option<Box<dyn InnerGameState>>;
	fn cameras_and_effect_and_renderer(&mut self) -> CamerasEffectRenderer {
		(None, None, None, None)
	}
	fn clean(&mut self, _window: &mut Window) {
	}
}

pub struct GameState {
	inner_state: Box<dyn InnerGameState>,
	mouse_buttons: MouseButtons,
}

impl GameState {
	pub fn new(inner_state: Box<dyn InnerGameState>) -> Self {
		Self {
			inner_state,
			mouse_buttons: MouseButtons::default(),
		}
	}
}

impl State for GameState {
	fn step(&mut self, window: &mut Window) {
		self.mouse_buttons.step();
		for mut event in window.events().iter() {
			if let WindowEvent::Key(Key::Escape, _, _) = event.value {
				event.inhibited = true;
			} else if let WindowEvent::MouseButton(button, action, _) = event.value {
				self.mouse_buttons.update(&button, &action);
			}
		}

		while let Some(new_state) = self.inner_state.step(window, &self.mouse_buttons) {
			self.inner_state.clean(window);
			self.inner_state = new_state;
			self.inner_state.init(window);
		}
	}

	fn cameras_and_effect_and_renderer(&mut self) -> CamerasEffectRenderer {
		self.inner_state.cameras_and_effect_and_renderer()
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MouseAction {
	Pressed,
	Held,
	Released,
	Free,
}

impl MouseAction {
	fn step(&self) -> Self {
		if self == &Self::Pressed {
			Self::Held
		} else if self == &Self::Released {
			Self::Free
		} else {
			*self
		}
	}

	fn update(&self, action: &Action) -> Self {
		if action == &Action::Press {
			Self::Pressed
		} else {
			Self::Released
		}
	}
}

pub struct MouseButtons {
	lmb: MouseAction,
	rmb: MouseAction,
	mmb: MouseAction,
}

impl MouseButtons {
	fn step(&mut self) {
		self.lmb = self.lmb.step();
		self.rmb = self.rmb.step();
		self.mmb = self.mmb.step();
	}

	fn update(&mut self, button: &MouseButton, action: &Action) {
		match button {
			MouseButton::Button1 => self.lmb = self.lmb.update(action),
			MouseButton::Button2 => self.rmb = self.rmb.update(action),
			MouseButton::Button3 => self.mmb = self.mmb.update(action),
			_ => {}
		};
	}
}

impl Default for MouseButtons {
	fn default() -> Self {
		MouseButtons {
			lmb: MouseAction::Free,
			rmb: MouseAction::Free,
			mmb: MouseAction::Free,
		}
	}
}
