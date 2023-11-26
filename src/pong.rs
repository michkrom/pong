extern crate pancurses;
use pancurses::Window;

use crate::rect::Rect;

pub struct Pong<'w> {
    window: &'w Window,
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
    paddle2dy: f64,
    serve_countdown: i32,
}

impl<'w> Pong<'w> {
    pub fn new(window: &Window) -> Pong {
        let mut pong = Pong {
            window: window,
            my: 0.0,
            mx: 0.0,
            score1: 0,
            score2: 0,
            hits: 0,
            ball: Default::default(),
            dx: 0.0,
            dy: 0.0,
            paddle1: Default::default(),
            paddle2: Default::default(),
            paddle1dy: 0.0,
            paddle2dy: 0.0,
            serve_countdown: 0,
        };
        pong.resize();
        pong.serve_reset(true);
        pong
    }

    pub fn resize(&mut self) {
        self.my = self.window.get_max_y() as f64;
        self.mx = self.window.get_max_x() as f64;
        self.paddle1.x = 0.0;
        self.paddle1.y = 0.0;
        self.paddle1.dx = 1.0;
        self.paddle1.dy = 9.0;
        self.paddle1.c = '|';
        self.paddle2.x = self.mx-1.0;
        self.paddle2.y = 0.0;
        self.paddle2.dx = 1.0;
        self.paddle2.dy = 9.0;
        self.paddle2.c = '|';
        self.ball.c = 'O';
        self.ball.dx = 1.0;
        self.ball.dy = 1.0;
        self.window.clear();
        self.serve_reset(true);
        self.update();
    }

    fn ball_dx(&self) -> f64 {
        let v = if self.hits < 4 {
            1.0
        } else if self.hits < 12 {
            1.6
        } else {
            2.1
        };
        v * self.mx / (self.mx + self.my) / 2.0
    }

    fn ball_dy(&self, n: i32) -> f64 {
        let v =
        match n {
            -4 => 2.0,
            -3 => 1.4,
            -2 => 1.0,
            -1 => 0.7,
            0 => 0.0,
            1 => -0.7,
            2 => -1.0,
            3 => -1.4,
            4 => -2.0,
            _ => 0.0
        };
        v * self.my / (self.mx + self.my) / 2.0
    }

    pub fn serve_reset(&mut self, player1: bool) {
        self.paddle1
            .moveto(self.window, 0.0, self.my / 2.0 - self.paddle1.dy / 2.0);
        self.paddle2
            .moveto(self.window, self.mx - 1.0, self.my / 2.0 - self.paddle2.dy / 2.0);
        self.ball.moveto(
            &self.window,
            if player1 { 1.0 } else { self.mx - 2.0 },
            self.my / 2.0,
        );
        self.hits = 0;
        self.dy = self.ball_dy(0);
        self.dx = self.ball_dx();
        if player1 { self.dx *= -1.0; }
        self.serve_countdown = 100;
    }

    pub fn update(&mut self) {
        self.paddle1.redraw(self.window);
        self.paddle2.redraw(self.window);
        if self.serve_countdown > 0 {
            self.serve_countdown -= 1;
        } else {
            (self.dx, self.dy) = self.ball.moveby(self.window, self.dx, self.dy);
        }
        if self.ball.overlap(&self.paddle1) {
            self.hits += 1;
            self.dx = self.ball_dx();
            self.dy = self.ball_dy((self.paddle1.y+self.paddle1.dy/2.0-self.ball.y) as i32);
        } else if self.ball.overlap(&self.paddle2) {
            self.hits += 1;
            self.dx = -self.ball_dx();
            self.dy = self.ball_dy((self.paddle2.y+self.paddle1.dy/2.0-self.ball.y) as i32);
        } else if self.ball.x <= 0.0 {
            self.score2 += 1;
            self.serve_reset(true);
        } else if self.ball.x + self.ball.dx >= self.mx {
            self.score1 += 1;
            self.serve_reset(false);
        }
        self.window.mvaddstr(0, (self.mx/2.0) as i32, format!("{:02} {:02} {:02}", self.score1, self.hits, self.score2));
    }

    pub fn on_key(&mut self, key: char) {
        match key {
            'z' => {
                self.paddle1.moveby(self.window, 0.0, self.paddle1dy);
                self.paddle1dy += 0.25;
            }
            'a' => {
                self.paddle1.moveby(self.window, 0.0, -self.paddle1dy);
                self.paddle1dy += 0.25;
            }
            'm' => {
                self.paddle2.moveby(self.window, 0.0, self.paddle2dy);
                self.paddle2dy += 0.25;
            }
            'k' => {
                self.paddle2.moveby(self.window, 0.0, -self.paddle2dy);
                self.paddle2dy += 0.25;
            }
            _ => {}
        }
    }

    pub fn on_none_input(&mut self) {
        self.paddle1dy = 1.0;
        self.paddle2dy = 1.0;
    }
}
