extern crate piston_window;

use piston_window::*;

mod snake;

mod food;

mod game;
use game::{new_state, State};

pub const COLS: u32 = 20;
pub const ROWS: u32 = 20;
pub const CELLSIZE: f64 = 10.0;
pub const HEIGHT: f64 = CELLSIZE * COLS as f64;
pub const WIDTH: f64 = CELLSIZE * ROWS as f64;

fn main() {
	let mut window: PistonWindow = WindowSettings::new(
		"Snake",
		[CELLSIZE * (ROWS as f64), CELLSIZE * (COLS as f64)],
	)
	.exit_on_esc(false)
	.build()
	.unwrap();

	let mut glyphs = window.load_font("FiraSans-Regular.ttf").unwrap();

	let mut game: Box<State> = new_state();

	let mut eventloop = Events::new(EventSettings::new()).ups(10);
	while let Some(event) = eventloop.next(&mut window) {
		if let Some(_) = event.update_args() {
			game = game.update();
		}
		if let Some(_) = event.render_args() {
			game = game.render(&event, &mut window, &mut glyphs);
		}
		if let Some(button) = event.button_args() {
			if button.state == ButtonState::Press {
				game = game.buttonpress(button);
			}
		}
	}
}
