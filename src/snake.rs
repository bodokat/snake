extern crate piston_window;

use piston_window::*;

use std::collections::LinkedList;
use std::iter::FromIterator;

use crate::food::Food;
use crate::{COLS, ROWS};

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    parts: LinkedList<SnakePiece>,
    pub growing: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
    None,
}

#[derive(Debug, Clone)]
struct SnakePiece(u32, u32);

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        return Snake {
            direction: Direction::None,
            parts: LinkedList::from_iter((vec![SnakePiece(x, y)]).into_iter()),
            growing: false,
        };
    }
    pub fn render(&self, ctx: Context, g2d: &mut G2d) {
        for SnakePiece(x, y) in self.parts.iter() {
            let (x, y) = ((*x) as f64, (*y) as f64);
            rectangle(
                [0.0, 1.0, 0.5, 1.0],
                [x * 10.0, y * 10.0, 10.0, 10.0],
                ctx.transform,
                g2d,
            );
        }
        // render head
        let SnakePiece(x, y) = self.parts.front().expect("There is no Snake");
        let (x, y) = ((*x) as f64, (*y) as f64);
        rectangle(
            [1.0, 1.0, 0.0, 1.0],
            [x * 10.0, y * 10.0, 10.0, 10.0],
            ctx.transform,
            g2d,
        );
    }
    pub fn update(&mut self) -> bool {
        let SnakePiece(mut x, mut y) = (*self.parts.front().expect("There is no Snake")).clone();
        match self.direction {
            Direction::Up => {
                if y != 0 {
                    y = y - 1
                }
            }
            Direction::Down => {
                if y != ROWS - 1 {
                    y = y + 1
                }
            }
            Direction::Left => {
                if x != 0 {
                    x = x - 1
                }
            }
            Direction::Right => {
                if x != COLS - 1 {
                    x = x + 1
                }
            }
            Direction::None => {
                return false;
            }
        }
        if self.collision(x, y) {
            return true;
        }
        self.parts.push_front(SnakePiece(x, y));
        if self.growing {
            self.growing = false;
        } else {
            self.parts.pop_back();
        }
        return false;
    }
    pub fn change_direction(&mut self, d: Direction) {
        self.direction = match d {
            Direction::Right if self.direction != Direction::Left => Direction::Right,
            Direction::Left if self.direction != Direction::Right => Direction::Left,
            Direction::Up if self.direction != Direction::Down => Direction::Up,
            Direction::Down if self.direction != Direction::Up => Direction::Down,
            _ => self.direction.clone(),
        }
    }
    pub fn has_eaten(&self, food: &Food) -> bool {
        let head = self.parts.front().expect("There is no Snake");
        return head.0 == food.0 && head.1 == food.1;
    }
    pub fn collision(&self, x: u32, y: u32) -> bool {
        self.parts.iter().any(|part| part.0 == x && part.1 == y)
    }
}
