use std::{io::{Write}, collections::binary_heap::Iter};

pub struct CrosstermHandler <W: Write> {
  buffer: W
}

impl <W> Write for CrosstermHandler<W>
where W: Write {
  fn flush(&mut self) -> std::io::Result<()> {
    self.buffer.flush()
  }

  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.buffer.write(buf)
  }
}

impl <W> CrosstermHandler<W>
where W: Write {
  pub fn new(buffer: W) -> Self {
    CrosstermHandler { buffer }
  }
}
