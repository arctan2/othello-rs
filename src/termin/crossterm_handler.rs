use crate::termin::buffer::Cell;
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    queue,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

use super::window::WindowRef;

pub struct CrosstermHandler<W: Write> {
    buffer: W,
}

impl<W> Write for CrosstermHandler<W>
where
    W: Write,
{
    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }
}

impl<W> CrosstermHandler<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> Self {
        CrosstermHandler { buffer }
    }

    pub fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut bg = Color::Reset;
        let mut fg = Color::Reset;
        let mut attr = Attribute::Reset;
        let mut last_pos: Option<(u16, u16)> = None;

        for (x, y, cell) in content {
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                queue!(self.buffer, MoveTo(x, y))?;
            }
            last_pos = Some((x, y));

            if attr != cell.attr {
                queue!(
                    self.buffer,
                    SetAttribute(cell.attr),
                    SetBackgroundColor(bg),
                    SetForegroundColor(fg)
                )?;
                attr = cell.attr;
            }

            if bg != cell.bg {
                queue!(self.buffer, SetBackgroundColor(cell.bg))?;
                bg = cell.bg;
            }

            if fg != cell.fg {
                queue!(self.buffer, SetForegroundColor(cell.fg))?;
                fg = cell.fg;
            }

            queue!(self.buffer, Print(&cell.symbol))?;
        }

        queue!(
            self.buffer,
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
            SetAttribute(Attribute::Reset)
        )?;

        Ok(())
    }

    pub fn draw_window(&mut self, win: &WindowRef) -> io::Result<()> {
        self.draw(win.inner().buffer().to_vec(win.abs_pos()).into_iter())
    }

    pub fn event(&self) -> Event {
        read().unwrap()
    }

    pub fn getch(&self) -> KeyCode {
        loop {
            match self.event() {
                Event::Key(k) => return k.code,
                _ => (),
            }
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}
