use std::collections::VecDeque;
use termren::Pixel;

const SNAKE_COLOR: termren::Color = termren::Color { r: 120, g: 220, b: 120 };
const SNAKE_PX: [char; 2] = ['░', '░'];

pub struct Snake {
  pub head: Pixel,
  pub body: VecDeque<Pixel>,
  direction: Direction,
  prev_direction: Direction,
  next_direction: Option<Direction>,
}

impl Snake {
  pub fn new(x: u16, y: u16, length: u16) -> Self {
    let mut body: Vec<Pixel> = vec![];
    for i in 0..length {
      body.push((x, y + i + 1, SNAKE_COLOR, SNAKE_PX).into());
    }
    Self {
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

  fn simulate_serpentine(&mut self) -> Pixel {
    if self.prev_direction == self.direction && self.next_direction.is_some() {
      self.direction = std::mem::replace(&mut self.next_direction, None).unwrap();
    }
    self.prev_direction = self.direction;
    match self.direction {
      Direction::Up => (self.head.x, self.head.y - 1, SNAKE_COLOR, SNAKE_PX).into(),
      Direction::Right => (self.head.x + 2, self.head.y, SNAKE_COLOR, SNAKE_PX).into(),
      Direction::Down => (self.head.x, self.head.y + 1, SNAKE_COLOR, SNAKE_PX).into(),
      Direction::Left => (self.head.x - 2, self.head.y, SNAKE_COLOR, SNAKE_PX).into(),
    }
  }

  pub fn steer(&mut self, event: termren::Event) {
    if let termren::Event::Key(key_event) = event {
      if let Some(dir) = Direction::from_event(key_event) {
        if self.direction != self.prev_direction && dir.inverse() != self.direction {
          self.next_direction = Some(dir);
        } else if dir.inverse() != self.prev_direction {
          self.direction = dir;
        }
      }

      if key_event.code == termren::KeyCode::Char('e') {
        let next_head = self.simulate_serpentine();
        self.body.push_front(std::mem::replace(&mut self.head, next_head));
      }
      if key_event.code == termren::KeyCode::Char('q') {
        self.body.pop_back();
      }
    }
  }

  pub fn serpentine(&mut self) {
    let next_head = self.simulate_serpentine();
    self.body.push_front(std::mem::replace(&mut self.head, next_head));
    self.body.pop_back();
  }

  pub fn try_eat(&mut self, food: &mut super::Food, ctx: &termren::Context) {
    if self.head == food.pixel {
      food.relocate(ctx);
      if let Some(last_tail) = self.body.back() {
        self.body.push_back(*last_tail);
      } else {
        self.body.push_back(self.head)
      }
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

  pub fn from_event(key_event: termren::KeyEvent) -> Option<Direction> {
    match key_event.code {
      termren::KeyCode::Up => Some(Direction::Up),
      termren::KeyCode::Right => Some(Direction::Right),
      termren::KeyCode::Down => Some(Direction::Down),
      termren::KeyCode::Left => Some(Direction::Left),
      _ => None,
    }
  }
}
