macro_rules! choose_side_win {
  ($terminal:ident, $heading:expr, $label:expr) => {{
    use crossterm::{style::{Color, Attribute}, event::KeyCode};
    use crate::{
      termin::{
        window::{Window, draw_elements, Position::*},
        elements::Text
      },
      menu::Return
    };
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

macro_rules! render_seq {
  ($win:expr,{x: $x:expr,y: $y:expr},$first:expr,$first_gap:expr,$($el:expr,$gap:expr),+) => {
    $first.set_xy($x, $y);
    let (mut prev_left, mut prev_bottom) = ($x, $first.height() + $y + $first_gap);
    $win.render_element(&$first);
    $(
      $el.set_xy(prev_left, prev_bottom);
      $win.draw_element(&$el);
      (prev_left, prev_bottom) = ($el.x(), $el.height() + $el.y() + $gap);
    )+
  };
  ($win:expr,{x: $x:expr,y: $y:expr, gap: $gap:expr},$first:expr,$($el:expr),+) => {
    $first.set_xy($x, $y);
    let (mut prev_left, mut prev_bottom) = ($first.x(), $first.height() + $first.y() + $gap);
    $win.render_element(&$first);
    $(
      $el.set_xy(prev_left, prev_bottom);
      $win.draw_element(&$el);
      (prev_left, prev_bottom) = ($el.x(), $el.height() + $el.y() + $gap);
    )+
  };
}

pub(crate) use {choose_side_win, render_seq};