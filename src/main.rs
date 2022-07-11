use termren;
use std::{rc::Rc, cell::RefCell};

fn main() {
  let pixels: Vec<termren::Pixel> = vec![
    // H
    (1, 1).into(),
    (1, 2).into(),
    (1, 3).into(),
    (1, 4).into(),
    (1, 5).into(),

    (1, 3).into(),
    (3, 3).into(),
    (5, 3).into(),
    (7, 3).into(),
    (9, 3).into(),

    (11, 1).into(),
    (11, 2).into(),
    (11, 3).into(),
    (11, 4).into(),
    (11, 5).into(),

    // E
    (17, 1).into(),
    (17, 2).into(),
    (17, 3).into(),
    (17, 4).into(),
    (17, 5).into(),

    (19, 1).into(),
    (21, 1).into(),
    (23, 1).into(),
    (25, 1).into(),

    (19, 3).into(),
    (21, 3).into(),
    (23, 3).into(),

    (19, 5).into(),
    (21, 5).into(),
    (23, 5).into(),
    (25, 5).into(),

    // L
    (31, 1).into(),
    (31, 2).into(),
    (31, 3).into(),
    (31, 4).into(),
    (31, 5).into(),

    (33, 5).into(),
    (35, 5).into(),
    (37, 5).into(),
    (39, 5).into(),

    // L
    (45, 1).into(),
    (45, 2).into(),
    (45, 3).into(),
    (45, 4).into(),
    (45, 5).into(),

    (47, 5).into(),
    (49, 5).into(),
    (51, 5).into(),
    (53, 5).into(),

    // O
    (59, 1).into(),
    (59, 2).into(),
    (59, 3).into(),
    (59, 4).into(),
    (59, 5).into(),

    (61, 1).into(),
    (63, 1).into(),
    (65, 1).into(),
    (67, 1).into(),

    (61, 5).into(),
    (63, 5).into(),
    (65, 5).into(),
    (67, 5).into(),

    (69, 1).into(),
    (69, 2).into(),
    (69, 3).into(),
    (69, 4).into(),
    (69, 5).into()
  ];
  let game = SnakeGame {
    pixels,
  };
  termren::Renderer::new(Rc::new(RefCell::new(game))).set_fps(60).run();
}

struct SnakeGame {
  pixels: Vec<termren::Pixel>,
}

impl termren::EventHandler for SnakeGame {
  fn update(
    &mut self,
    event_option: Option<termren::Event>
  ) -> termren::SingleOrMulti<termren::Pixel> {
    let mut dx: i16 = 0;
    let mut dy: i16 = 0;
    if let Some(event) = event_option {
      if termren::is_key_pressed(event, termren::KeyCode::Up) {
        dy = -1;
      } else if termren::is_key_pressed(event, termren::KeyCode::Down) {
        dy = 1;
      } else if termren::is_key_pressed(event, termren::KeyCode::Left) {
        dx = -2;
      } else if termren::is_key_pressed(event, termren::KeyCode::Right) {
        dx = 2;
      }
    }
    for px in self.pixels.iter_mut() {
      px.x = if px.x <= 1 && dx < 0 {
        px.x
      } else { (px.x as i16 + dx) as u16 };
      px.y = if px.y <= 1 && dy < 0 {
        px.y
      } else { (px.y as i16 + dy) as u16 };
    }

    termren::SingleOrMulti::Multi(&self.pixels)
  }
}
