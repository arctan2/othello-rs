use super::buffer::{
  Buffer, Rect
};

pub trait Element {
  fn draw(&self, buf: &mut Buffer); 
}

mod rectangle;
mod text;
mod input;

pub use rectangle::Rectangle;
pub use text::Text;
pub use input::{InputBox};

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

      pub fn set_pos(&mut self, x: u16, y: u16) {
        self.rect.x = x;
        self.rect.y = y;
      }

      pub fn set_size(&mut self, width: u16, height: u16) {
        self.rect.width = width;
        self.rect.height = height;
      }
    }
  };
}

pub(crate) use impl_setters;