mod screens;

use raylib::prelude::*;
use screens::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

const TARGET_FPS: u32 = 60;

fn main() {
    let title = "raylib [core] example - basic screen manager";

    // Screen Transition State
    let mut current_screen = GameScreen::LOGO;

    // Game Screen Objects
    let mut logo_obj = LogoObject::new();
    let title_obj = TitleObject::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut game_obj = GameObject::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut ending_obj = EndingObject::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Raylib
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(title)
        .build();

    // Set FPS
    rl.set_target_fps(TARGET_FPS);

    while !rl.window_should_close() {
        // ON HANDLE_EVENT
        current_screen = match current_screen {
            GameScreen::LOGO => logo_obj.handle_event(&rl).unwrap_or(current_screen),
            GameScreen::TITLE => title_obj.handle_event(&rl).unwrap_or(current_screen),
            GameScreen::GAMEPLAY => game_obj.handle_event(&rl).unwrap_or(current_screen),
            GameScreen::ENDING => ending_obj.handle_event(&rl).unwrap_or(current_screen),
        };

        let mut d = rl.begin_drawing(&thread);

        {
            d.clear_background(Color::RAYWHITE);

            // ON PAINT
            match current_screen {
                GameScreen::LOGO => logo_obj.paint(&mut d),
                GameScreen::TITLE => title_obj.paint(&mut d),
                GameScreen::GAMEPLAY => game_obj.paint(&mut d),
                GameScreen::ENDING => ending_obj.paint(&mut d),
            }
        }
    }
}
