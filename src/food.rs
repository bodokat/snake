extern crate piston_window;
extern crate rand;

use piston_window::*;

use crate::snake::Snake;
use crate::{COLS, ROWS};

#[derive(Debug)]
pub struct Food(pub u32, pub u32);

impl Food {
    pub fn render(&self, ctx: Context, g2d: &mut G2d) {
        let (x, y) = (f64::from(self.0), f64::from(self.1));
        rectangle(
            [0.5, 0.5, 0.5, 1.0],
            [x * 10.0, y * 10.0, 10.0, 10.0],
            ctx.transform,
            g2d,
        );
    }
    pub fn randomize(snake: &Snake) -> Food {
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();
        let mut result: Food;
        let max_count = 100;
        let mut count = 0;
        loop {
            let x: u32 = rng.gen_range(0, ROWS);
            let y: u32 = rng.gen_range(0, COLS);
            result = Food(x, y);

            count += 1;
            if count > max_count {
                panic!(
                    "Could not find suitable position for food after {} tries!",
                    count
                );
            }

            if !snake.collision(x, y) {
                break;
            }
        }
        result
    }
}
