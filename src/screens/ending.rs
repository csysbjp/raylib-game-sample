use raylib::prelude::*;

use super::GameScreen;

pub struct EndingObject {
    screen_width: i32,
    screen_height: i32,
}

impl EndingObject {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        EndingObject {
            screen_width,
            screen_height,
        }
    }

    pub fn handle_event(&mut self, rl: &RaylibHandle) -> Option<GameScreen> {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_gesture_detected(Gesture::GESTURE_TAP)
        {
            return Some(GameScreen::TITLE);
        }
        None
    }

    pub fn paint<'a>(&self, d: &mut RaylibDrawHandle<'a>) {
        d.draw_rectangle(0, 0, self.screen_width, self.screen_height, Color::BLUE);
        d.draw_text("ENDING SCREEN", 20, 20, 40, Color::DARKBLUE);
        d.draw_text(
            "PRESS ENTER or TAP to RETURN to TITLE SCREEN",
            120,
            220,
            20,
            Color::DARKBLUE,
        );
    }
}
