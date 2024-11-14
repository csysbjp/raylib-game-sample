use rand::prelude::*;
use raylib::prelude::*;

enum GameScreen {
    LOGO = 0,
    TITLE,
    GAMEPLAY,
    ENDING,
}

const MAX_BUILDINGS: usize = 100;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let title = "raylib [core] example - basic screen manager";

    let mut current_screen = GameScreen::LOGO;
    let mut frames_counter = 0;

    let mut player = Rectangle {
        x: 400f32,
        y: 280f32,
        width: 40f32,
        height: 40f32,
    };

    let mut buildings: Vec<Rectangle> = Vec::with_capacity(MAX_BUILDINGS);
    let mut build_colors: Vec<Color> = Vec::with_capacity(MAX_BUILDINGS);
    let mut spacing: i32 = 0;

    let mut rng = rand::thread_rng();

    for i in 0..MAX_BUILDINGS {
        let width = rng.gen_range(50..200).as_f32();
        let height = rng.gen_range(100..800).as_f32();
        let y = screen_height.as_f32() - 130.0f32 - height;
        let x = -6000.0f32 + spacing.as_f32();

        buildings.push(Rectangle {
            width,
            height,
            y,
            x,
        });

        spacing += width as i32;

        build_colors.push(Color {
            r: rng.gen_range(200..240),
            g: rng.gen_range(200..240),
            b: rng.gen_range(200..250),
            a: 255,
        });
    }

    let mut camera = Camera2D {
        target: Vector2 {
            x: player.x + 20.0f32,
            y: player.y + 20.0f32,
        },
        offset: Vector2 {
            x: screen_width.as_f32() / 2.0f32,
            y: screen_height.as_f32() / 2.0f32,
        },
        rotation: 0.0f32,
        zoom: 1.0f32,
    };

    let (mut rl, thread) = init()
        .size(screen_width, screen_height)
        .title(title)
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        match current_screen {
            GameScreen::LOGO => {
                frames_counter += 1;

                if frames_counter > 120 {
                    current_screen = GameScreen::TITLE;
                }
            }
            GameScreen::TITLE => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_gesture_detected(Gesture::GESTURE_TAP)
                {
                    current_screen = GameScreen::GAMEPLAY;
                }
            }
            GameScreen::GAMEPLAY => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_gesture_detected(Gesture::GESTURE_TAP)
                {
                    current_screen = GameScreen::ENDING;
                }

                if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                    player.x += 2.0f32;
                } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                    player.x -= 2.0f32;
                }

                camera.target = Vector2 {
                    x: player.x + 20f32,
                    y: player.y + 20f32,
                };

                if rl.is_key_down(KeyboardKey::KEY_A) {
                    camera.rotation += 1f32;
                } else if rl.is_key_down(KeyboardKey::KEY_S) {
                    camera.rotation -= 1f32;
                }

                if camera.rotation > 40.0f32 {
                    camera.rotation = 40.0f32;
                } else if camera.rotation < -40.0f32 {
                    camera.rotation = -40.0f32;
                }

                camera.zoom += rl.get_mouse_wheel_move() * 0.05f32;

                if camera.zoom > 3.0f32 {
                    camera.zoom = 3.0f32;
                } else if camera.zoom < 0.1f32 {
                    camera.zoom = 0.1f32;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_R) {
                    camera.zoom = 1.0f32;
                    camera.rotation = 1.0f32;
                }
            }
            GameScreen::ENDING => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_gesture_detected(Gesture::GESTURE_TAP)
                {
                    current_screen = GameScreen::TITLE;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);

        {
            d.clear_background(Color::RAYWHITE);

            match current_screen {
                GameScreen::LOGO => {
                    d.draw_text("LOGO SCREEN", 20, 20, 40, Color::LIGHTGRAY);
                    d.draw_text("WAIT 2 SECONDS...", 290, 220, 20, Color::GRAY);
                }
                GameScreen::TITLE => {
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
                GameScreen::GAMEPLAY => {
                    {
                        let mut d2d = d.begin_mode2D(camera);
                        d2d.draw_rectangle(-6000, 320, 13000, 8000, Color::DARKGRAY);

                        for i in 0..MAX_BUILDINGS {
                            d2d.draw_rectangle_rec(buildings[i], build_colors[i]);
                        }

                        d2d.draw_rectangle(
                            camera.target.x as i32 - 10,
                            camera.target.y as i32 - 20,
                            20,
                            40,
                            Color::RED,
                        );
                        d2d.draw_line(
                            camera.target.x as i32,
                            -screen_height * 10,
                            camera.target.x as i32,
                            screen_height * 10,
                            Color::GREEN,
                        );
                        d2d.draw_line(
                            -screen_height * 10,
                            camera.target.y as i32,
                            screen_width * 10,
                            camera.target.y as i32,
                            Color::GREEN,
                        );
                    }

                    d.draw_text("SCREEN AREA", 640, 10, 20, Color::RED);

                    d.draw_rectangle(0, 0, screen_width, 5, Color::RED);
                    d.draw_rectangle(0, 5, 5, screen_height - 10, Color::RED);
                    d.draw_rectangle(screen_width - 5, 5, 5, screen_height - 10, Color::RED);
                    d.draw_rectangle(0, screen_height - 5, screen_width, 5, Color::RED);

                    let skyblue_faded = d.gui_fade(Color::SKYBLUE, 0.5f32);
                    d.draw_rectangle(10, 10, 250, 113, skyblue_faded);
                    d.draw_rectangle_lines(10, 10, 250, 113, Color::BLUE);

                    d.draw_text("Free 2d camera controls:", 20, 20, 10, Color::BLACK);
                    d.draw_text("- Right/Left to move Offset", 40, 40, 10, Color::DARKGRAY);
                    d.draw_text("- Mouse Wheel to Zoom in-out", 40, 60, 10, Color::DARKGRAY);
                    d.draw_text("- A / S to Rotate", 40, 80, 10, Color::DARKGRAY);
                    d.draw_text(
                        "- R to reset Zoom and Rotation",
                        40,
                        100,
                        10,
                        Color::DARKGRAY,
                    );
                }
                GameScreen::ENDING => {
                    d.draw_rectangle(0, 0, screen_width, screen_height, Color::BLUE);
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
        }
    }
}
