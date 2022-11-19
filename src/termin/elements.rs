use super::{buffer::{
  Buffer, Rect
}, window::Position};

pub trait Element {
  fn draw(&self, buf: &mut Buffer); 
}

mod rectangle;
mod text;
mod input;

pub use rectangle::Rectangle;
pub use text::Text;
pub use input::InputBox;

macro_rules! impl_setters {
  ($struct_name:ident) => {
    impl $struct_name {
      pub fn position(mut self, x: u16, y: u16) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
      }

      pub fn size(mut self, width: u16, height: u16) -> Self {
        self.rect.width = width;
        self.rect.height = height;
        self
      }

      pub fn set_xy(&mut self, x: u16, y: u16) {
        self.rect.x = x;
        self.rect.y = y;
      }

      pub fn set_xy_rel(&mut self, mut dx: i16, mut dy: i16) {
        let (x, y) = self.rect.get_xy();
        dx += x as i16;
        dy += y as i16;
        if dx < 0 { dx = 0; }
        if dy < 0 { dy = 0; }
        self.set_xy(dx as u16, dy as u16);
      }

      pub fn set_size(&mut self, width: u16, height: u16) {
        self.rect.width = width;
        self.rect.height = height;
      }

      pub fn set_position(&mut self, rect: Rect, pos: Position) {
        match pos {
          Position::Coord(x, y) => self.set_xy(x, y),
          Position::Center { h, v } => {
            let (x, y) = rect.get_center_start_pos(self.rect.clone());

            if h && v {
              self.set_xy(x, y);
            } else {
              if h {
                self.set_xy(x, self.rect.y);
              } else {
                self.set_xy(self.rect.x, y);
              }
            }
          }
        }
      }
    }
  };
}

pub(crate) use impl_setters;