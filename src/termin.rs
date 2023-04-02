pub mod buffer;
pub mod crossterm_handler;
pub mod elements;
pub mod terminal_window;
pub mod window;

use self::crossterm_handler::CrosstermHandler;
use self::terminal_window::Terminal;
use self::window::Window;
use crossterm::terminal;

pub fn root<W: std::io::Write>(handler: CrosstermHandler<W>) -> Terminal<W> {
    use std::process;
    match terminal::size() {
        Ok((w, h)) => Terminal::new(Window::default().size(w.into(), h.into()), handler),
        _ => {
            println!("unable to get terminal size");
            process::exit(1);
        }
    }
}
