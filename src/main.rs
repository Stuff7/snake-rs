mod snake;
mod food;

use termren;
use std::{rc::Rc, cell::RefCell};
use snake::Snake;
use food::Food;

fn main() {
  let game = SnakeGame::new(Snake::new(6, 10, 16));
  termren::Renderer::new(Rc::new(RefCell::new(game))).set_fps(60).run();
}

const TPS: f64 = 1.0 / 30.0;

struct SnakeGame {
  snake: Snake,
  food: Food,
  pixels: Vec<termren::Pixel>,
  timedelta_passed: f64,
}

impl SnakeGame {
  pub fn new(snake: Snake) -> Self {
    Self {
      snake,
      food: Food::new(20, 10),
      pixels: vec![],
      timedelta_passed: 0.0,
    }
  }
}

impl termren::EventHandler for SnakeGame {
  fn update(
    &mut self,
    ctx: &termren::Context,
  ) -> (termren::Group<termren::Pixel>, Option<String>) {
    self.timedelta_passed = self.timedelta_passed + ctx.timedelta;
    if let Some(event) = ctx.event {
      self.snake.steer(event);
    }
    if self.timedelta_passed >= TPS {
      self.timedelta_passed = 0.0;
      self.snake.try_eat(&mut self.food, ctx);
      self.snake.serpentine();
    }

    self.pixels = vec![
      self.food.pixel,
    ];

    self.pixels.extend(self.snake.body.iter());

    (
      termren::Group::Multi(self.pixels.as_slice()),
      Some(format!(
        "{pos_start}Score: {green}{snake_length}{reset} \
        Food: {green}({food_x}, {food_y}){reset} \
        Snake: {green}({snake_x}, {snake_y}){reset} \
        Console: {green}({csl_x}, {csl_y}){reset}",
        pos_start = termren::console::seq::CURSOR_START,
        green = termren::console::seq::fg_rgb(100, 200, 50),
        snake_length = self.snake.len(),
        reset = termren::console::seq::RESET,
        food_x = self.food.pixel.x,
        food_y = self.food.pixel.y,
        snake_x = self.snake.head.x,
        snake_y = self.snake.head.y,
        csl_x = ctx.console_size.width,
        csl_y = ctx.console_size.height,
      ))
    )
  }
}
