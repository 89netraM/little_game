use kiss3d::{
	conrod::{
		color::{Color, Colorable},
		position::{Positionable, Sizeable},
		widget::{self, Widget},
		widget_ids,
		Borderable,
		Labelable,
	},
	window::Window,
};

use super::{super::GAME_NAME, InnerGameState};

pub struct MenuState {
	ui_ids: UiIds,
}

impl MenuState {
	pub fn new(window: &mut Window) -> Self {
		Self {
			ui_ids: UiIds::new(window.conrod_ui_mut().widget_id_generator()),
		}
	}
}

impl InnerGameState for MenuState {
	fn step(&mut self, window: &mut Window) -> Option<Box<dyn InnerGameState>> {
		let start_clicked;
		let exit_clicked;
		{
			let mut ui = window.conrod_ui_mut().set_widgets();

			widget::Text::new(GAME_NAME)
				.font_size(75)
				.color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.mid_top_with_margin(150.0)
				.center_justify()
				.set(self.ui_ids.title, &mut ui);

			start_clicked = widget::Button::new()
				.color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.hover_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.press_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.border_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.label("Start New")
				.label_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.w(200.0)
				.mid_bottom_with_margin(200.0)
				.set(self.ui_ids.start_button, &mut ui);
			exit_clicked = widget::Button::new()
				.color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.hover_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.press_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.border_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.label("Exit")
				.label_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.w(200.0)
				.mid_bottom_with_margin(100.0)
				.set(self.ui_ids.exit_button, &mut ui);

			widget::Text::new("Created by Mårten Åsberg for the 4MB Game Jam")
				.font_size(12)
				.color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.mid_bottom_with_margin(10.0)
				.center_justify()
				.set(self.ui_ids.credits, &mut ui);
		}

		if start_clicked.was_clicked() {
			Some(Box::new(super::PlayingState::new(window, 0, (0, 0))))
		} else if exit_clicked.was_clicked() {
			window.close();
			None
		} else {
			None
		}
	}
}

widget_ids! {
	struct UiIds {
		title,
		start_button,
		exit_button,
		credits,
	}
}
