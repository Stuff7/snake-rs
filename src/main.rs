mod snake;

use termren;
use std::{rc::Rc, cell::RefCell};
use snake::Snake;

fn main() {
  let game = SnakeGame {
    snake: Snake::new(6, 10),
  };
  termren::Renderer::new(Rc::new(RefCell::new(game))).set_fps(30).run();
}

struct SnakeGame {
  snake: Snake,
}

impl termren::EventHandler for SnakeGame {
  fn update(
    &mut self,
    event_option: Option<termren::Event>
  ) -> termren::SingleOrMulti<termren::Pixel> {
    if let Some(event) = event_option {
      self.snake.steer(event);
    }

    self.snake.serpentine();

    termren::SingleOrMulti::Multi(&self.snake.body)
  }
}
