extern crate pancurses;

use pancurses::{curs_set, endwin, initscr, noecho, Input, Window};

fn draw_rect(window: &Window, x: i32, y: i32, dx: i32, dy: i32, c: char) {
    for i in 0..dy {
        for j in 0..dx {
            window.mvaddch(y + i, x + j, c);
        }
    }
}

#[derive(Default)]
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

#[derive(Default)]
struct Pong {
  my: f64, 
  mx: f64,  
  score1: i32, 
  score2: i32,
  hits: i32,
  ball: Rect,
  dx: f64,
  dy: f64,
  paddle1: Rect,
  paddle2: Rect,
  paddle1dy: f64,
  paddle2dy: f64
}

impl Pong {
  fn resize(&mut self, window: &Window) {
    self.my = window.get_max_y() as f64;
    self.mx = window.get_max_x() as f64;
    self.paddle1.dx = 1.0;
    self.paddle1.dy = 7.0;
    self.paddle1.c = '|';
    self.paddle2.dx = 1.0;
    self.paddle2.dy = 7.0;
    self.paddle2.c = '|';
    self.ball.c = 'O';
    self.ball.dx = 1.0;
    self.ball.dy = 1.0;
  }

  fn ball_dx(&self) -> f64 {
    let v = 
    if self.hits < 4 {
      1.0
    } else if self.hits < 12 {
      1.6
    } else {
      2.1
    };
    v * self.mx / (self.mx + self.my)
  }

  fn serve_reset(&mut self, window: &Window, player1: bool) {
    self.paddle1.moveto(&window, 0.0, self.my / 2.0 - self.paddle1.dy / 2.0);
    self.paddle2.moveto(&window, self.mx - 1.0, self.my / 2.0 - self.paddle2.dy / 2.0);
    self.ball.moveto(&window, if player1 { 1.0 } else { self.mx - 1.0 }, self.my / 2.0);
    self.hits = 0;
    self.dy = self.my / (self.mx + self.my);
    self.dx = self.ball_dx()
  }
  
  fn update(&mut self, window: &Window) {
    (self.dx, self.dy) = self.ball.moveby(window, self.dx, self.dy);
    self.paddle1.redraw(window);
    self.paddle2.redraw(window);
    if self.ball.overlap(&self.paddle1) {
      self.hits += 1;
      self.dx = self.ball_dx();
    } else if self.ball.overlap(&self.paddle2) {
        self.hits += 1;
        self.dx = -self.ball_dx();
    } else if self.ball.x <= 0.0 {
        self.score2 += 1;
        self.serve_reset(window, true);
    } else if self.ball.x + self.ball.dx >= self.mx {
        self.score1 += 1;
        self.serve_reset(window, false);
      }
  }

  fn on_key(&mut self, window: &Window, key: char) {
    match key {
      'z' => {
          self.paddle1.moveby(window, 0.0, self.paddle1dy);
          self.paddle1dy += 0.25;
      }
      'a' => {
        self.paddle1.moveby(window, 0.0, -self.paddle1dy);
        self.paddle1dy += 0.25;
      }
      'm' => {
        self.paddle2.moveby(window, 0.0, self.paddle2dy);
        self.paddle2dy += 0.25;
      }
      'k' => {
        self.paddle2.moveby(window, 0.0, -self.paddle2dy);
        self.paddle2dy += 0.25;
      }
      _ => {}
  }
}
}

fn main() {
    let window = initscr();
    window.keypad(true);
    //nodelay(&window, true);
    //raw();
    noecho();
    curs_set(0);
    window.timeout(10);
    let mut pong : Pong = Default::default();
    pong.resize(&window);
    pong.serve_reset(&window, true);
    loop {
        pong.update(&window);
        match window.getch() {
            Some(Input::Character(c)) => { pong.on_key(&window, c); },
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => {
              pong.paddle1dy = 1.0;
              pong.paddle2dy = 1.0;
            }
        }
    }
    endwin();
}
