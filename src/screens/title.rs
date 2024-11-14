use raylib::prelude::*;

use super::GameScreen;

pub struct TitleObject {
    pub screen_width: i32,
    pub screen_height: i32,
}

impl TitleObject {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        TitleObject {
            screen_width,
            screen_height,
        }
    }

    pub fn handle_event(&self, rl: &RaylibHandle) -> Option<GameScreen> {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_gesture_detected(Gesture::GESTURE_TAP)
        {
            return Some(GameScreen::GAMEPLAY);
        }

        None
    }

    pub fn paint<'a>(&self, d: &mut RaylibDrawHandle<'a>) {
        let screen_width = self.screen_width;
        let screen_height = self.screen_height;

        d.draw_rectangle(0, 0, screen_width, screen_height, Color::GREEN);
        d.draw_text("TITLE SCREEN", 20, 20, 40, Color::DARKGREEN);
        d.draw_text(
            "PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN",
            120,
            220,
            20,
            Color::DARKGREEN,
        );
    }
}
