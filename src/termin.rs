pub mod crossterm_handler;
pub mod window;
pub mod elements;
pub mod buffer;

use crossterm::terminal;
use self::window::Window;
use self::crossterm_handler::CrosstermHandler;

pub fn root<'a, W: std::io::Write>(handler: &'a CrosstermHandler<W>) -> Window<'a, W> {
  use std::process;
  match terminal::size() {
    Ok((w, h)) => Window::default(handler).size(w, h),
    _ => {
      println!("unable to get terminal size");
      process::exit(1); 
    }
  }
}
