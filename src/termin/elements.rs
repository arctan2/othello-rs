use super::{
    buffer::{Buffer, Rect},
    window::Position,
};

pub trait Element {
    fn draw(&self, buf: &mut Buffer);
}

mod input;
mod rectangle;
mod text;

pub use input::InputWindow;
pub use rectangle::Rectangle;
pub use text::Text;

macro_rules! impl_setters {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn xy(mut self, x: u32, y: u32) -> Self {
                self.rect.x = x;
                self.rect.y = y;
                self
            }

            pub fn size(mut self, width: u32, height: u32) -> Self {
                self.rect.width = width;
                self.rect.height = height;
                self
            }

            pub fn xy_rel(mut self, mut dx: i16, mut dy: i16) -> Self {
                self.set_xy_rel(dx, dy);
                self
            }

            pub fn set_xy(&mut self, x: u32, y: u32) {
                self.rect.x = x;
                self.rect.y = y;
            }

            pub fn set_xy_rel(&mut self, mut dx: i16, mut dy: i16) {
                let (x, y) = self.rect.get_xy();
                dx += x as i16;
                dy += y as i16;
                if dx < 0 {
                    dx = 0;
                }
                if dy < 0 {
                    dy = 0;
                }
                self.set_xy(dx as u32, dy as u32);
            }

            pub fn set_size(&mut self, width: u32, height: u32) {
                self.rect.width = width;
                self.rect.height = height;
            }

            pub fn position(mut self, rect: Rect, pos: Position) -> Self {
                self.set_position(rect, pos);
                self
            }

            pub fn set_position(&mut self, rect: Rect, pos: Position) {
                match pos {
                    Position::Coord(x, y) => self.set_xy(rect.x + x, rect.y + y),
                    Position::CenterB | Position::CenterV | Position::CenterH => {
                        let (x, y) = rect.get_center_start_pos(self.rect.clone());

                        match pos {
                            Position::CenterB => self.set_xy(x, y),
                            Position::CenterH => self.set_xy(x, self.rect.y),
                            Position::CenterV => self.set_xy(self.rect.x, y),
                            _ => (),
                        }
                    }
                }
            }

            pub fn width(&self) -> u32 {
                self.rect.width
            }

            pub fn height(&self) -> u32 {
                self.rect.height
            }

            pub fn x(&self) -> u32 {
                self.rect.x
            }

            pub fn y(&self) -> u32 {
                self.rect.y
            }
        }
    };
}

pub(crate) use impl_setters;
