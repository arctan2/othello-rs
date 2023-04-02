mod custom_elements;
mod game;
mod menu;
mod termin;

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use game::macros::choose_side_win;
use game::online_lobby::Online;
use menu::{Menu, Return};
use std::io::stdout;
use std::thread;
use std::time::Duration;
use termin::crossterm_handler::CrosstermHandler;
use termin::elements::Text;
use termin::terminal_window::TerminalHandler;
use termin::window::{Position::*, Window};

use crate::game::offline_game::play_offline;
use crate::termin::elements::InputWindow;

fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn change_name(terminal: &mut TerminalHandler, ctx: &mut Ctx) -> Return {
    let name = terminal.handle_input(|handler, root| -> String {
        root.clear();
        handler.draw_window(&root).unwrap();
        let mut input_win = root.new_child(Window::default().size(50, 10)).xy_rel(2, 2);
        let label = Text::default().text("new name: ").xy(0, 2);
        let mut input = InputWindow::from(
            &mut input_win,
            Window::default()
                .xy(label.x() + label.width(), label.y())
                .size(21, 1),
        )
        .start_text((0, 0))
        .max_len(20);

        input_win.draw_element(&label);
        input_win.draw_text("Change Name", CenterH);
        input_win.render();

        handler.draw_window(&root).unwrap();

        let new_name = input.read_string(handler);
        input_win.delete();
        new_name
    });

    ctx.name = name;
    Return::None
}

struct Ctx {
    name: String,
}

fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

    let mut terminal = termin::root(CrosstermHandler::new(stdout()));

    let mut game_ctx = Ctx {
        name: "Player".to_string(),
    };

    let mut menu_map = Menu::<Ctx>::new("Main Menu")
        .routine(&|menu, ctx| {
            let mut s = "Welome ".to_string();
            s.push_str(&ctx.name);
            menu.heading.set_text(&s);
            menu.heading.width_fit();
        })
        .sub_menu(
            "play game",
            Menu::<Ctx>::new("Play game")
                .sub_menu(
                    "offline",
                    Menu::<Ctx>::new("Offline")
                        .action("1 player", &|terminal, ctx| -> Return {
                            play_offline(terminal, ctx, 1)
                        })
                        .action("2 player", &|terminal, ctx| -> Return {
                            play_offline(terminal, ctx, 2)
                        })
                        .back("back"),
                )
                .sub_menu(
                    "online",
                    Menu::<Ctx>::new("Online")
                        .action("create game", &|terminal, ctx| -> Return {
                            let host_side = choose_side_win!(
                                terminal,
                                "Create Game Online",
                                "Choose your side:"
                            );
                            Online::new(&mut terminal.root)
                                .set_player(host_side, &ctx.name)
                                .create_and_start(terminal);
                            Return::None
                        })
                        .action("join game", &|terminal, ctx| -> Return {
                            Online::new(&mut terminal.root)
                                .set_player(0 as char, &ctx.name)
                                .join_and_start(terminal);
                            Return::None
                        })
                        .back("back"),
                )
                .back("back"),
        )
        .action("change name", &change_name)
        .back("quit");

    menu_map.run(&mut terminal, &mut game_ctx);

    execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use super::*;

    #[test]
    fn scroll() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();
        let mut terminal = termin::root(CrosstermHandler::new(stdout()));

        let mut win = terminal.root.new_child(
            Window::default()
                .size(50, 10)
                .scoll_size(80, 50)
                .bg(crossterm::style::Color::Blue),
        );
        let mut tex = String::new();

        for i in 0..50 {
            tex += &("some text ".to_string() + &i.to_string() + "\n");
        }

        let t = Text::default().text(&tex).size(20, 50);

        win.draw_element(&t);
        win.render();
        terminal.refresh().unwrap();

        loop {
            match terminal.getch() {
                KeyCode::Esc => break,
                KeyCode::Left => {
                    win.set_scroll_xy_rel(-1, 0);
                    win.render();
                    terminal.refresh().unwrap();
                }
                KeyCode::Right => {
                    win.set_scroll_xy_rel(1, 0);
                    win.render();
                    terminal.refresh().unwrap();
                }
                KeyCode::Up => {
                    win.set_scroll_xy_rel(0, -1);
                    win.render();
                    terminal.refresh().unwrap();
                }
                KeyCode::Down => {
                    win.set_scroll_xy_rel(0, 1);
                    win.render();
                    terminal.refresh().unwrap();
                }
                _ => (),
            }
        }

        execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    #[test]
    fn change_name_fn() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();
        let mut terminal = termin::root(CrosstermHandler::new(stdout()));
        let mut game_ctx = Ctx {
            name: "Player".to_string(),
        };

        change_name(&mut terminal, &mut game_ctx);

        execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
