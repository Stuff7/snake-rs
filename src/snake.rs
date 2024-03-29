use std::collections::VecDeque;
use termren::Pixel;

const SNAKE_COLOR: termren::Color = termren::Color { r: 120, g: 220, b: 120 };
const SNAKE_PX: &str = "░░";

#[derive(Clone)]
pub struct Snake {
  pub alive: bool,
  pub head: Pixel,
  pub body: VecDeque<Pixel>,
  direction: Direction,
  prev_direction: Direction,
  next_direction: Option<Direction>,
}

impl Snake {
  pub fn new(x: u16, y: u16, length: u16) -> Self {
    let mut body: Vec<Pixel> = vec![];
    for _ in 0..length {
      body.push((x, y, SNAKE_COLOR, SNAKE_PX).into());
    }
    Self {
      alive: true,
      head: (x, y, SNAKE_COLOR, SNAKE_PX).into(),
      body: body.into(),
      direction: Direction::Right,
      prev_direction: Direction::Right,
      next_direction: None,
    }
  }

  pub fn len(&self) -> usize {
    self.body.len()
  }

  fn try_move(&mut self, ctx: &termren::Context) -> Pixel {
    if self.prev_direction == self.direction && self.next_direction.is_some() {
      self.direction = std::mem::replace(&mut self.next_direction, None).unwrap();
    }
    self.prev_direction = self.direction;
    let next_head: Pixel = match self.direction {
      Direction::Up => (self.head.x, self.head.y - 1, SNAKE_COLOR, SNAKE_PX).into(),
      Direction::Right => (self.head.x + 2, self.head.y, SNAKE_COLOR, SNAKE_PX).into(),
      Direction::Down => (self.head.x, self.head.y + 1, SNAKE_COLOR, SNAKE_PX).into(),
      Direction::Left => (self.head.x - 2, self.head.y, SNAKE_COLOR, SNAKE_PX).into(),
    };
    self.alive = !(
      next_head.x <= 0 ||
      next_head.x > ctx.console_size.width ||
      next_head.y <= 0 ||
      next_head.y > ctx.console_size.height
    ) && !self.body.iter().any(|part| next_head == *part);

    next_head
  }

  pub fn steer(&mut self, key_event: &termren::KeyEvent) {
    if !self.alive {
      return;
    }
    if let Some(dir) = Direction::from_key_code(key_event.code) {
      if self.direction != self.prev_direction && dir.inverse() != self.direction {
        self.next_direction = Some(dir);
      } else if dir.inverse() != self.prev_direction {
        self.direction = dir;
      }
    }
  }

  pub fn serpentine(&mut self, ctx: &termren::Context) {
    if self.alive {
      let next_head = self.try_move(ctx);
      self.body.push_front(std::mem::replace(&mut self.head, next_head));
      self.body.pop_back();
    }
  }

  pub fn try_eat(&mut self, food: &mut super::Food, ctx: &termren::Context) {
    if self.alive && self.head == food.pixel {
      if let Some(last_tail) = self.body.back() {
        self.body.push_back(*last_tail);
      } else {
        self.body.push_back(self.head)
      }
      food.relocate(ctx);
    }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  pub fn inverse(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Right => Direction::Left,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
    }
  }

  pub fn from_key_code(key_code: termren::KeyCode) -> Option<Direction> {
    match key_code {
      termren::KeyCode::Up => Some(Direction::Up),
      termren::KeyCode::Right => Some(Direction::Right),
      termren::KeyCode::Down => Some(Direction::Down),
      termren::KeyCode::Left => Some(Direction::Left),
      _ => None,
    }
  }
}
