#![allow(dead_code)]
extern crate piston_window;
use piston_window::*;

use crate::food::Food;
use crate::snake::Direction;
use crate::snake::Snake;
use crate::{COLS, HEIGHT, ROWS, WIDTH};

pub struct Game {
    world: World,
    state: Box<dyn State>,
}

struct World {
    score: u32,
    // highscore: u32,
    snake: Snake,
    food: Food,
}

trait State {
    fn update(&self, world: &mut World) -> Option<Box<dyn State>>;
    fn render(&self, world: &World, event: &Event, window: &mut PistonWindow, glyphs: &mut Glyphs);
    fn buttonpress(&self, world: &mut World, button: ButtonArgs) -> Option<Box<dyn State>>;
}

struct Paused;
struct Playing;
struct GameOver;
struct Menu;

pub fn new_game() -> Game {
    let snake = Snake::new(ROWS / 2, COLS / 2);
    let food = Food::randomize(&snake);
    Game {
        world: World {
            score: 0,
            snake,
            food,
        },
        state: Box::new(Playing),
    }
}

impl Game {
    pub fn update(&mut self) {
        if let Some(new) = self.state.update(&mut self.world) {
            self.state = new;
        }
    }
    pub fn render(&mut self, event: &Event, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        self.state.render(&self.world, event, window, glyphs);
    }
    pub fn buttonpress(&mut self, button: ButtonArgs) {
        if let Some(new) = self.state.buttonpress(&mut self.world, button) {
            self.state = new;
        }
    }
}

impl State for Playing {
    fn update(&self, world: &mut World) -> Option<Box<dyn State>> {
        let collision = world.snake.update();
        if collision {
            return Some(Box::new(GameOver));
        }
        if world.snake.has_eaten(&world.food) {
            world.score += 1;
            if world.score == ROWS * COLS - 1 {
                // if snake fills the board, win
                // if we did not do this, Food::randomize would loop forever
                return Some(Box::new(GameOver));
            }
            world.snake.growing = true;
            world.food = Food::randomize(&world.snake);
        }
        None
    }
    fn render(&self, world: &World, event: &Event, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            world.snake.render(context, graphics);
            world.food.render(context, graphics);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16)
                .draw(
                    &world.score.to_string(),
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(10.0, 10.0),
                    graphics,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
    }
    fn buttonpress(&self, world: &mut World, button: ButtonArgs) -> Option<Box<State>> {
        if let Button::Keyboard(key) = button.button {
            match key {
                Key::Escape => return Some(Box::new(Paused)),
                Key::Up => world.snake.change_direction(Direction::Up),
                Key::Down => world.snake.change_direction(Direction::Down),
                Key::Left => world.snake.change_direction(Direction::Left),
                Key::Right => world.snake.change_direction(Direction::Right),
                _ => {}
            }
        }
        None
    }
}

impl State for Paused {
    fn update(&self, _world: &mut World) -> Option<Box<dyn State>> {
        None
    }
    fn render(&self, world: &World, event: &Event, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            world.snake.render(context, graphics);
            world.food.render(context, graphics);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16)
                .draw(
                    &world.score.to_string(),
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
    }
    fn buttonpress(&self, _world: &mut World, button: ButtonArgs) -> Option<Box<dyn State>> {
        if let Button::Keyboard(key) = button.button {
            if key == Key::Escape {
                return Some(Box::new(Playing));
            }
        }
        None
    }
}

impl State for GameOver {
    fn update(&self, _world: &mut World) -> Option<Box<dyn State>> {
        None
    }
    fn render(&self, world: &World, event: &Event, window: &mut PistonWindow, glyphs: &mut Glyphs) {
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
                    format!("Your score was: {}", world.score).as_str(),
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
    }
    fn buttonpress(&self, world: &mut World, button: ButtonArgs) -> Option<Box<dyn State>> {
        if button.button == Button::Keyboard(Key::Space) {
            world.score = 0;
            world.snake = Snake::new(ROWS / 2, COLS / 2);
            world.food = Food::randomize(&world.snake);
            return Some(Box::new(Playing));
        }
        None
    }
}
