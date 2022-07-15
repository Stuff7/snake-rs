mod snake;
mod food;

use termren;
use std::{rc::Rc, cell::RefCell};
use snake::Snake;
use food::Food;

fn main() {
  let game = SnakeGame::new(Snake::new(6, 10, 10));
  termren::Renderer::new(Rc::new(RefCell::new(game))).set_fps(60).run();
}

const TPS: f64 = 1.0 / 30.0;

const GREEN: &str = "\x1b[38;2;100;200;50m";
const RED: &str = "\x1b[38;2;200;100;50m";

struct SnakeGame {
  snake: Snake,
  snake_clone: Snake,
  food: Food,
  pixels: Vec<termren::Pixel>,
  timedelta_passed: f64,
}

impl SnakeGame {
  pub fn new(snake: Snake) -> Self {
    Self {
      snake_clone: snake.clone(),
      snake,
      food: Food::new(20, 10),
      pixels: vec![],
      timedelta_passed: 0.0,
    }
  }

  pub fn handle_input(&mut self, key_event: &termren::KeyEvent) {
    self.snake.steer(key_event);
    if !self.snake.alive {
      if key_event.code == termren::KeyCode::Char('r') {
        self.snake = self.snake_clone.clone();
      }
    }
  }
}

impl termren::EventHandler for SnakeGame {
  fn update(
    &mut self,
    ctx: &termren::Context,
  ) -> (termren::Group<termren::Pixel>, Option<String>) {
    self.timedelta_passed = self.timedelta_passed + ctx.timedelta;

    if let Some(key_event) = termren::event::to_key_event(ctx.event) {
      self.handle_input(&key_event);
    }

    if self.timedelta_passed >= TPS {
      self.timedelta_passed = 0.0;
      self.snake.try_eat(&mut self.food, ctx);
      self.snake.serpentine(ctx);
    }

    self.pixels = vec![
      self.food.pixel,
    ];

    self.pixels.extend(self.snake.body.iter());

    (
      termren::Group::Multi(self.pixels.as_slice()),
      {
        let mut ui = format!(
          "{pos_start}Score: {GREEN}{score}{reset} \
          Food: {GREEN}({food_x}, {food_y}){reset} \
          Snake: {GREEN}({snake_x}, {snake_y}){reset}",
          pos_start = termren::console::seq::CURSOR_START,
          score = self.snake.len(),
          reset = termren::console::seq::RESET,
          food_x = self.food.pixel.x,
          food_y = self.food.pixel.y,
          snake_x = self.snake.head.x,
          snake_y = self.snake.head.y,
        );
        if !self.snake.alive {
          ui.push_str(&format!(
            " {bold}{RED}Game Over!{reset} Press {GREEN}R{reset} to play again",
            bold = termren::console::seq::BOLD,
            reset = termren::console::seq::RESET,
          ))
        }
        Some(ui)
      }
    )
  }
}
