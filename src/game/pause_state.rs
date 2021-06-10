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

use super::{super::GAME_NAME, playing_state::SavedPlayingState, InnerGameState};

pub struct PauseState {
	ui_ids: UiIds,
	playing_state: SavedPlayingState,
}

impl PauseState {
	pub fn new(window: &mut Window, playing_state: SavedPlayingState) -> Self {
		Self {
			ui_ids: UiIds::new(window.conrod_ui_mut().widget_id_generator()),
			playing_state,
		}
	}
}

impl InnerGameState for PauseState {
	fn step(&mut self, window: &mut Window) -> Option<Box<dyn InnerGameState>> {
		let continue_clicked;
		let menu_clicked;
		let me_clicked;
		let game_jam_clicked;
		{
			let mut ui = window.conrod_ui_mut().set_widgets();

			widget::Text::new(GAME_NAME)
				.font_size(75)
				.color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.mid_top_with_margin(150.0)
				.center_justify()
				.set(self.ui_ids.title, &mut ui);

			continue_clicked = widget::Button::new()
				.color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.hover_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.press_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.border_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.label("Continue")
				.label_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.w(200.0)
				.mid_bottom_with_margin(200.0)
				.set(self.ui_ids.continue_button, &mut ui);
			menu_clicked = widget::Button::new()
				.color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.hover_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.press_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.border_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.label("Main Menu")
				.label_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.w(200.0)
				.mid_bottom_with_margin(100.0)
				.set(self.ui_ids.menu_button, &mut ui);

			widget::Text::new("Created by")
				.font_size(12)
				.color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.bottom_left_with_margin(10.0)
				.set(self.ui_ids.created_text, &mut ui);
			me_clicked = widget::Button::new()
				.color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.hover_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.press_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.border_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.label("Mårten Åsberg")
				.label_font_size(12)
				.label_color(Color::Rgba(0.0, 0.5, 1.0, 1.0))
				.right_from(self.ui_ids.created_text, 15.0)
				.set(self.ui_ids.me_link, &mut ui);
			widget::Text::new("for the")
				.font_size(12)
				.color(Color::Rgba(1.0, 1.0, 1.0, 1.0))
				.right_from(self.ui_ids.me_link, 17.0)
				.set(self.ui_ids.for_text, &mut ui);
			game_jam_clicked = widget::Button::new()
				.color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.hover_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.press_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.border_color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
				.label("4MB game jam")
				.label_font_size(12)
				.label_color(Color::Rgba(0.0, 0.5, 1.0, 1.0))
				.right_from(self.ui_ids.for_text, 30.0)
				.set(self.ui_ids.game_jam_link, &mut ui);
		}

		if continue_clicked.was_clicked() {
			Some(Box::new(super::PlayingState::restore(
				window,
				&self.playing_state,
			)))
		} else if menu_clicked.was_clicked() {
			Some(Box::new(super::MenuState::new(window)))
		} else if me_clicked.was_clicked() {
			let _ = webbrowser::open("https://åsberg.net/");
			None
		} else if game_jam_clicked.was_clicked() {
			let _ = webbrowser::open("https://itch.io/jam/4mb");
			None
		} else {
			None
		}
	}
}

widget_ids! {
	struct UiIds {
		title,
		continue_button,
		menu_button,
		created_text,
		me_link,
		for_text,
		game_jam_link,
	}
}
