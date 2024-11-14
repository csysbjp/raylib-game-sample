use raylib::prelude::*;

use super::GameScreen;

pub struct LogoObject {
    frames_counter: i32,
}

impl LogoObject {
    pub fn new() -> Self {
        LogoObject { frames_counter: 0 }
    }

    pub fn handle_event(&mut self, _rl: &RaylibHandle) -> Option<GameScreen> {
        self.frames_counter += 1;

        if self.frames_counter > 120 {
            return Some(GameScreen::TITLE);
        }

        None
    }

    pub fn paint<'a>(&self, d: &mut RaylibDrawHandle<'a>) {
        d.draw_text("LOGO SCREEN", 20, 20, 40, Color::LIGHTGRAY);
        d.draw_text("WAIT 2 SECONDS...", 290, 220, 20, Color::GRAY);
    }
}
