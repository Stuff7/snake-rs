use termren::{Pixel, Color};

const FOOD_COLOR: Color = Color::new(220, 150, 120);

pub struct Food {
  pub pixel: Pixel,
}

impl Food {
  pub fn new(x: u16, y: u16) -> Self {
    Self { pixel: (x, y, FOOD_COLOR, '▓').into() }
  }

  pub fn relocate(&mut self, ctx: &termren::Context) {
    let (width, height) = (ctx.console_size.width, ctx.console_size.height);
    self.pixel.randomize_position(
      width,
      height,
    );
    let x = self.pixel.x + 1;
    self.pixel.x = std::cmp::max(
      if x % 2 == 0 {x} else {x + 1},
      if (width - 1) % 2 == 0 {width - 1} else {width - 2}
    );
    self.pixel.y = std::cmp::max(self.pixel.y + 1, height - 1);
  }
}
