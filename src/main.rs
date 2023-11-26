extern crate pancurses;
use pancurses::{curs_set, endwin, initscr, noecho, Input};

mod pong;
mod rect;

fn main() {
    let window = initscr();
    window.keypad(true);
    //nodelay(&window, true);
    //raw();
    noecho();
    curs_set(0);
    window.timeout(10);
    let mut game = pong::Pong::new(&window);
    loop {
        game.update();
        match window.getch() {
            Some(Input::Character(c)) => {
                game.on_key(c);
            }
            Some(Input::KeyDC) => break,
            Some(Input::KeyResize) => {
                game.resize();
            }
            Some(input) => {
                window.mvaddstr(0, 0, &format!("{:?}", input));
            }
            None => game.on_none_input(),
        }
    }
    endwin();
}
