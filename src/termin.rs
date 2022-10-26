pub mod crossterm_handler;
pub mod window;
pub mod elements;
pub mod buffer;
pub mod terminal_window;

use crossterm::terminal;
use self::terminal_window::Terminal;
use self::window::Window;
use self::crossterm_handler::CrosstermHandler;

pub fn root<W: std::io::Write>(handler: CrosstermHandler<W>) -> Terminal<W> {
  use std::process;
  match terminal::size() {
    Ok((w, h)) => Terminal::new(Window::default().size(w, h), handler),
    _ => {
      println!("unable to get terminal size");
      process::exit(1); 
    }
  }
}
