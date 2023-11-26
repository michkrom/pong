extern crate pancurses;
use pancurses::Window;

use crate::rect::Rect;

pub struct Pong<'w> {
    w: &'w Window,
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
            w: window,
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
        self.my = self.w.get_max_y() as f64;
        self.mx = self.w.get_max_x() as f64;
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
        let v = if self.hits < 4 {
            1.0
        } else if self.hits < 12 {
            1.6
        } else {
            2.1
        };
        v * self.mx / (self.mx + self.my) / 2.0
    }

    pub fn serve_reset(&mut self, player1: bool) {
        self.paddle1
            .moveto(self.w, 0.0, self.my / 2.0 - self.paddle1.dy / 2.0);
        self.paddle2
            .moveto(self.w, self.mx - 1.0, self.my / 2.0 - self.paddle2.dy / 2.0);
        self.ball.moveto(
            &self.w,
            if player1 { 1.0 } else { self.mx - 2.0 },
            self.my / 2.0,
        );
        self.hits = 0;
        self.dy = self.my / (self.mx + self.my);
        self.dx = self.ball_dx();
        if player1 { self.dx *= -1.0; }
        self.serve_countdown = 100;
    }

    pub fn update(&mut self) {
        if self.serve_countdown > 0 {
            self.serve_countdown -= 1;
        } else {
            (self.dx, self.dy) = self.ball.moveby(self.w, self.dx, self.dy);
        }
        self.paddle1.redraw(self.w);
        self.paddle2.redraw(self.w);
        if self.ball.overlap(&self.paddle1) {
            self.hits += 1;
            self.dx = self.ball_dx();
        } else if self.ball.overlap(&self.paddle2) {
            self.hits += 1;
            self.dx = -self.ball_dx();
        } else if self.ball.x <= 0.0 {
            self.score2 += 1;
            self.serve_reset(true);
        } else if self.ball.x + self.ball.dx >= self.mx {
            self.score1 += 1;
            self.serve_reset(false);
        }
    }

    pub fn on_key(&mut self, key: char) {
        match key {
            'z' => {
                self.paddle1.moveby(self.w, 0.0, self.paddle1dy);
                self.paddle1dy += 0.25;
            }
            'a' => {
                self.paddle1.moveby(self.w, 0.0, -self.paddle1dy);
                self.paddle1dy += 0.25;
            }
            'm' => {
                self.paddle2.moveby(self.w, 0.0, self.paddle2dy);
                self.paddle2dy += 0.25;
            }
            'k' => {
                self.paddle2.moveby(self.w, 0.0, -self.paddle2dy);
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
