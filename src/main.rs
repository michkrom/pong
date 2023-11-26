extern crate pancurses;

use pancurses::{curs_set, endwin, initscr, noecho, Input, Window};

fn draw_rect(window: &Window, x: i32, y: i32, dx: i32, dy: i32, c: char) {
    for i in 0..dy {
        for j in 0..dx {
            window.mvaddch(y + i, x + j, c);
        }
    }
}

struct Rect {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    c: char,
}

impl Rect {
    fn draw(&self, window: &Window, c: char) {
        draw_rect(
            window,
            self.x as i32,
            self.y as i32,
            self.dx as i32,
            self.dy as i32,
            c,
        );
    }
    fn redraw(&self, window: &Window) {
        draw_rect(
            window,
            self.x as i32,
            self.y as i32,
            self.dx as i32,
            self.dy as i32,
            self.c,
        );
    }

    fn moveby(&mut self, window: &Window, dx: f64, dy: f64) -> (f64, f64) {
        self.draw(window, ' ');
        let (my, mx) = (window.get_max_y() as f64, window.get_max_x() as f64);
        self.x += dx;
        self.y += dy;
        let (mut ndx, mut ndy) = (dx, dy);
        if self.x < 0.0 {
            self.x = 0.0;
            ndx = -dx;
        }
        if self.x + self.dx > mx {
            self.x = mx - self.dx;
            ndx = -dx;
        }
        if self.y < 0.0 {
            self.y = 0.0;
            ndy = -dy;
        }
        if self.y + self.dy > my {
            self.y = my - self.dy;
            ndy = -dy;
        }
        self.draw(window, self.c);
        return (ndx, ndy);
    }

    fn moveto(&mut self, window: &Window, x: f64, y: f64) {
        self.draw(window, ' ');
        self.x = x;
        self.y = y;
        self.draw(window, self.c);
    }

    fn overlap(&self, other: &Rect) -> bool {
        if self.x + self.dx <= other.x || other.x + other.dx <= self.x {
            return false;
        }
        if self.y + self.dy <= other.y || other.y + other.dy <= self.y {
            return false;
        }
        true
    }
}

fn main() {
    let window = initscr();
    window.keypad(true);
    //nodelay(&window, true);
    //raw();
    noecho();
    curs_set(0);
    let mut ball = Rect {
        x: 0.0,
        y: 0.0,
        dx: 1.0,
        dy: 1.0,
        c: 'O',
    };
    let (my, mx) = (window.get_max_y() as f64, window.get_max_x() as f64);
    let (mut score1, mut score2) = (0, 0);
    let mut hits = 0;
    let mut dx = mx / (mx + my);
    let mut dy = my / (mx + my);
    let mut paddle1 = Rect {
        x: 0.0,
        y: my/2.0-3.0,
        dx: 1.0,
        dy: 7.0,
        c: '|',
    };
    let mut paddle2 = Rect {
        x: mx - 1.0,
        y: my/2.0-3.0,
        dx: 1.0,
        dy: 8.0,
        c: '|',
    };

    window.timeout(10);

    let (mut paddle1dy, mut paddle2dy) = (1.0, 1.0);

    loop {
        (dx, dy) = ball.moveby(&window, dx, dy);
        paddle1.redraw(&window);
        paddle2.redraw(&window);

        match window.getch() {
            Some(Input::Character(c)) => match c {
                'z' => {
                    paddle1.moveby(&window, 0.0, paddle1dy);
                    paddle1dy += 0.25;
                }
                'a' => {
                    paddle1.moveby(&window, 0.0, -paddle1dy);
                    paddle1dy += 0.25;
                }
                'm' => {
                    paddle2.moveby(&window, 0.0, paddle2dy);
                    paddle2dy += 0.25;
                }
                'k' => {
                    paddle2.moveby(&window, 0.0, -paddle2dy);
                    paddle2dy += 0.25;
                }
                _ => {}
            },
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => {
                paddle1dy = 1.0;
                paddle2dy = 1.0;
            }
        }
        if ball.overlap(&paddle1) {
            hits += 1;
            if (hits < 4) {
                dx = 1.0;
            } else if (hits < 12) {
                dx = 1.6;
            } else {
                dx = 2.1;
            }
            dx = 2.1;
            dx *= mx / (mx + my);
        } else if ball.overlap(&paddle2) {
            hits += 1;
            if (hits < 4) {
                dx = 1.0;
            } else if (hits < 12) {
                dx = 1.6;
            } else {
                dx = 2.1;
            }
            dx *= -mx / (mx + my);
        } else if ball.x <= 0.0 {
            score2 += 1;
            paddle1.moveto(&window, 0.0, my / 2.0 - paddle1.dy / 2.0);
            paddle2.moveto(&window, mx - 1.0, my / 2.0 - paddle2.dy / 2.0);
            ball.moveto(&window, 1.0, my / 2.0);
            hits = 0;
            dx = mx / (mx + my);            
        } else if ball.x + ball.dx >= mx {
            score1 += 1;
            paddle1.moveto(&window, 0.0, my / 2.0 - paddle1.dy / 2.0);
            paddle2.moveto(&window, mx - 1.0, my / 2.0 - paddle2.dy / 2.0);
            ball.moveto(&window, mx - 2.0, my / 2.0);
            hits = 0;
            dx = -mx / (mx + my);            
        }
    }
    endwin();
}
