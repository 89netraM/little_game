use instant::Instant;
use kiss3d::{
	conrod::{
		color::{Color, Colorable},
		position::Positionable,
		widget::{self, Widget},
		widget_ids,
	},
	event::Key,
	window::Window,
};

use super::{InnerGameState, MouseAction, MouseButtons};

pub struct StoryState {
	ui_ids: UiIds,
	page: usize,
	page_time: Instant,
	space_action: MouseAction,
}

impl StoryState {
	pub fn new(window: &mut Window) -> Self {
		Self {
			ui_ids: UiIds::new(window.conrod_ui_mut().widget_id_generator()),
			page: 6,
			page_time: Instant::now(),
			space_action: MouseAction::Free,
		}
	}
}

impl InnerGameState for StoryState {
	fn step(
		&mut self,
		window: &mut Window,
		mouse_buttons: &MouseButtons,
	) -> Option<Box<dyn InnerGameState>> {
		{
			let mut ui = window.conrod_ui_mut().set_widgets();
			let time = self.page_time.elapsed().as_secs_f32() * 0.5;

			widget::Text::new(match self.page {
				6 => "Wake up Agent!",
				5 => "I'm terribly sorry to tell you this, but...",
				4 => "You're in the maze",
				3 => "You know the drill.\nFind the key and get back to base as soon as possible.",
				2 => {
					"As you know, prolonged exposure usually doesn't end well.\nLuckily, the key \
					 should be in a nearby section."
				}
				1 => {
					"If you find any coins, take 'em with you.\nBut remember, coins aren't worth \
					 anything if don't\nmake it back."
				}
				0 => "Good luck!",
				_ => unreachable!(),
			})
			.font_size(24)
			.color(Color::Rgba(1.0, 1.0, 1.0, time.clamp(0.0, 1.0)))
			.middle()
			.set(self.ui_ids.story_text, &mut ui);

			widget::Text::new("LMB or Space to continue...")
				.font_size(20)
				.rgba(1.0, 1.0, 1.0, (time * 1.0 - 2.0).clamp(0.0, 1.0))
				.bottom_right_with_margin(50.0)
				.right_justify()
				.set(self.ui_ids.continue_text, &mut ui)
		}

		self.space_action = self.space_action.step().update(&window.get_key(Key::Space));

		if mouse_buttons.lmb == MouseAction::Pressed || self.space_action == MouseAction::Pressed {
			if self.page == 0 {
				Some(Box::new(super::PlayingState::new(window, 0)))
			} else {
				self.page -= 1;
				self.page_time = Instant::now();
				None
			}
		} else {
			None
		}
	}
}

widget_ids! {
	struct UiIds {
		story_text,
		continue_text,
	}
}
