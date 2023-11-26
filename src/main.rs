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
    let mut game = pong::Pong::new(&window);
    game.resize();
    game.serve_reset(true);
    loop {
        game.update();
        match window.getch() {
            Some(Input::Character(c)) => { game.on_key( c); },
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
