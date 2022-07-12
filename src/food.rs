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
      width / 2,
      height,
    );
    self.pixel.x = self.pixel.x * 2;
  }
}
