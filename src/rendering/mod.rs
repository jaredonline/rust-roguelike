extern crate tcod;
use self::tcod::{Console, background_flag};

use util::Point;

pub struct TcodRenderingComponent<'a> {
    pub console: &'a mut Console
}

impl<'a> TcodRenderingComponent<'a> {
    pub fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    pub fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as int, position.y as int, symbol, background_flag::Set);
    }

    pub fn after_render_new_frame(&mut self) {
        self.console.flush();
    }
}

