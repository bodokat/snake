extern crate piston_window;

use piston_window::*;

mod snake;

mod food;

mod game;
use game::{new_game, Game};

pub const COLS: u32 = 20;
pub const ROWS: u32 = 20;
pub const CELLSIZE: f64 = 10.0;
pub const HEIGHT: f64 = CELLSIZE * COLS as f64;
pub const WIDTH: f64 = CELLSIZE * ROWS as f64;

fn main() {
	let mut window: PistonWindow = WindowSettings::new(
		"Snake",
		[CELLSIZE * f64::from(ROWS), CELLSIZE * f64::from(COLS)],
	)
	.exit_on_esc(false)
	.build()
	.unwrap();

	let mut glyphs = window.load_font("FiraSans-Regular.ttf").unwrap();

	let mut game: Game = new_game();

	let mut eventloop = Events::new(EventSettings::new()).ups(10);
	while let Some(event) = eventloop.next(&mut window) {
		if event.update_args().is_some() {
			game.update();
		}
		if event.render_args().is_some() {
			game.render(&event, &mut window, &mut glyphs);
		}
		if let Some(button) = event.button_args() {
			if button.state == ButtonState::Press {
				game.buttonpress(button);
			}
		}
	}
}
