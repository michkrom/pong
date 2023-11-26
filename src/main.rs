extern crate pancurses;
use pancurses::{noecho, curs_set, initscr, endwin, Input};

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
    let mut game : pong::Pong = Default::default();
    game.resize(&window);
    game.serve_reset(&window, true);
    loop {
        game.update(&window);
        match window.getch() {
            Some(Input::Character(c)) => { game.on_key(&window, c); },
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => {
              game.on_none_input()
            }
        }
    }
    endwin();
}
