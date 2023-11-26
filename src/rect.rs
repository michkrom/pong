extern crate pancurses;
use pancurses::Window;

#[derive(Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub c: char
}

fn draw_rect(window: &Window, x: i32, y: i32, dx: i32, dy: i32, c: char) {
    for i in 0..dy {
        for j in 0..dx {
            window.mvaddch(y + i, x + j, c);
        }
    }
}

impl Rect {
    fn draw(&self, window: &Window, c: char) {
        draw_rect(
            window,
            self.x as i32,
            self.y as i32,
            self.dx as i32,
            self.dy as i32,
            c
        );
    }
    pub fn redraw(&self, window: &Window) {
        draw_rect(
            window,
            self.x as i32,
            self.y as i32,
            self.dx as i32,
            self.dy as i32,
            self.c
        );
    }

    pub fn moveby(&mut self, window: &Window, dx: f64, dy: f64) -> (f64, f64) {
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

    pub fn moveto(&mut self, window: &Window, x: f64, y: f64) {
        self.draw(window, ' ');
        self.x = x;
        self.y = y;
        self.draw(window, self.c);
    }

    pub fn overlap(&self, other: &Rect) -> bool {
        if self.x + self.dx <= other.x || other.x + other.dx <= self.x {
            return false;
        }
        if self.y + self.dy <= other.y || other.y + other.dy <= self.y {
            return false;
        }
        true
    }
}
