#![allow(dead_code)]
extern crate piston_window;
use piston_window::*;

use crate::food::Food;
use crate::snake::Direction;
use crate::snake::Snake;
use crate::{COLS, HEIGHT, ROWS, WIDTH};

pub trait State {
    fn update(self: Box<Self>) -> Box<dyn State>;
    fn render(
        self: Box<Self>,
        event: &Event,
        window: &mut PistonWindow,
        glyphs: &mut Glyphs,
    ) -> Box<dyn State>;
    fn buttonpress(self: Box<Self>, button: ButtonArgs) -> Box<dyn State>;
}

struct Paused {
    score: u32,
    snake: Snake,
    food: Food,
}
pub struct Playing {
    score: u32,
    snake: Snake,
    food: Food,
}
struct GameOver {
    score: u32,
}
struct Menu {
    score: u32,
}

pub fn new_state() -> Box<Playing> {
    let snake = Snake::new(ROWS / 2, COLS / 2);
    let food = Food::randomize(&snake);
    Box::new(Playing {
        score: 0,
        snake,
        food,
    })
}

impl State for Playing {
    fn update(mut self: Box<Self>) -> Box<dyn State> {
        let collision = self.snake.update();
        if collision {
            return Box::new(GameOver { score: self.score });
        }
        if self.snake.has_eaten(&self.food) {
            self.score += 1;
            if self.score == ROWS * COLS - 1 {
                // if snake fills the board, win
                // if we did not do this, Food::randomize would loop forever
                return Box::new(GameOver { score: self.score });
            }
            self.snake.growing = true;
            self.food = Food::randomize(&self.snake);
        }
        self
    }
    fn render(
        self: Box<Self>,
        event: &Event,
        window: &mut PistonWindow,
        glyphs: &mut Glyphs,
    ) -> Box<dyn State> {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            self.snake.render(context, graphics);
            self.food.render(context, graphics);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16)
                .draw(
                    &self.score.to_string(),
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(10.0, 10.0),
                    graphics,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
        self
    }
    fn buttonpress(mut self: Box<Self>, button: ButtonArgs) -> Box<State> {
        if let Button::Keyboard(key) = button.button {
            match key {
                Key::Escape => {
                    return Box::new(Paused {
                        score: self.score,
                        snake: self.snake,
                        food: self.food,
                    })
                }
                Key::Up => self.snake.change_direction(Direction::Up),
                Key::Down => self.snake.change_direction(Direction::Down),
                Key::Left => self.snake.change_direction(Direction::Left),
                Key::Right => self.snake.change_direction(Direction::Right),
                _ => {}
            }
        }
        self
    }
}

impl State for Paused {
    fn update(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn render(
        self: Box<Self>,
        event: &Event,
        window: &mut PistonWindow,
        glyphs: &mut Glyphs,
    ) -> Box<dyn State> {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            self.snake.render(context, graphics);
            self.food.render(context, graphics);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16)
                .draw(
                    &self.score.to_string(),
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(10.0, 10.0),
                    graphics,
                )
                .unwrap();
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                .draw(
                    "Paused",
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(HEIGHT / 2.0 - 20.0, 20.0),
                    graphics,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
        self
    }
    fn buttonpress(self: Box<Self>, button: ButtonArgs) -> Box<dyn State> {
        if let Button::Keyboard(key) = button.button {
            if key == Key::Escape {
                return Box::new(Playing {
                    score: self.score,
                    snake: self.snake,
                    food: self.food,
                });
            }
        }
        self
    }
}

impl State for GameOver {
    fn update(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn render(
        self: Box<Self>,
        event: &Event,
        window: &mut PistonWindow,
        glyphs: &mut Glyphs,
    ) -> Box<dyn State> {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0; 4], graphics);

            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw(
                    "Game over",
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(WIDTH / 2.0 - 75.0, HEIGHT / 3.0),
                    graphics,
                )
                .unwrap();
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16)
                .draw(
                    format!("Your score was: {}", self.score).as_str(),
                    glyphs,
                    &context.draw_state,
                    context
                        .transform
                        .trans(WIDTH / 2.0 - 75.0, HEIGHT * 2.0 / 3.0),
                    graphics,
                )
                .unwrap();

            glyphs.factory.encoder.flush(device);
        });
        self
    }
    fn buttonpress(self: Box<Self>, button: ButtonArgs) -> Box<dyn State> {
        if button.button == Button::Keyboard(Key::Space) {
            return new_state();
        }
        self
    }
}
