use crossterm::style::Color;
use termin::window::{Window, WindowOperations};

mod termin;

fn main() {
  let root: Window = termin::init_termin();

  root.set_cell_bg(5, 5, Color::Cyan);
  root.refresh();

  root.getch();
  
  termin::end_termin();
}