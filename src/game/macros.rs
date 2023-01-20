macro_rules! choose_side_win {
  ($terminal:ident, $heading:expr, $label:expr) => {{
    $terminal.clear();
    let mut win = $terminal.root.new_child(
      Window::default().size(30, 10).bg(Color::Rgb { r: 0, g: 180, b: 0 }).xy(2, 2)
    );
    let rect = win.rect();
    let mut cur_side = 'w';
    let mut white = Text::default().text("white").position(rect, CenterB).attr(Attribute::Underlined).fg(Color::White);
    let mut black = Text::default().text("black").position(rect, CenterB).fg(Color::Black);

    black.set_xy_rel(black.width() as i16, 1);
    white.set_xy_rel(-(white.width() as i16), 1);

    draw_elements!(win,
      Text::default()
        .text($heading)
        .fg(Color::Black)
        .attr(Attribute::Bold)
        .position(rect, CenterH)
        .xy_rel(0, 1),
      Text::default()
        .text($label)
        .xy_rel(2, 4)
        .fg(Color::Black),
      white, black
    );

    win.render();
    $terminal.refresh().unwrap();

    loop {
      let mut do_render = false;
      match $terminal.getch() {
        KeyCode::Left | KeyCode::Right => {
          match cur_side {
            'w' => {
              black.set_attr(Attribute::Underlined);
              white.set_attr(Attribute::Reset);
              cur_side = 'b';
              do_render = true;
            },
            'b' => {
              white.set_attr(Attribute::Underlined);
              black.set_attr(Attribute::Reset);
              cur_side = 'w';
              do_render = true;
            },
            _ => ()
          }
        },
        KeyCode::Enter => break,
        KeyCode::Esc => {
          win.delete();
          return Return::None
        }
        _ => ()
      }
      if do_render {
        draw_elements!(win, white, black);
        $terminal.handler.draw_window(&win).unwrap();
        $terminal.flush().unwrap();
      }
    }

    win.delete();
    cur_side
  }};
}

pub(crate) use choose_side_win;