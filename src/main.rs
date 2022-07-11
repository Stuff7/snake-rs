mod snake;

use termren;
use std::{rc::Rc, cell::RefCell};
use snake::Snake;

fn main() {
  let game = SnakeGame::new(Snake::new(6, 10, 16));
  termren::Renderer::new(Rc::new(RefCell::new(game))).set_fps(60).run();
}

const TPS: f64 = 1.0 / 12.0;

struct SnakeGame {
  snake: Snake,
  timedelta_passed: f64,
}

impl SnakeGame {
  pub fn new(snake: Snake) -> Self {
    Self { snake, timedelta_passed: 0.0 }
  }
}

impl termren::EventHandler for SnakeGame {
  fn update(
    &mut self,
    event_option: Option<termren::Event>,
    timedelta: f64,
  ) -> termren::Group<termren::Pixel> {
    self.timedelta_passed = self.timedelta_passed + timedelta;
    if let Some(event) = event_option {
      self.snake.steer(event);
    }
    if self.timedelta_passed >= TPS {
      self.timedelta_passed = 0.0;
      self.snake.serpentine();
    }

    termren::Group::Multi(&self.snake.body)
  }
}
