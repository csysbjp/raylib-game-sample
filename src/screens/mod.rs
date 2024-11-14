mod ending;
mod gameplay;
mod logo;
mod title;

pub use ending::*;
pub use gameplay::*;
pub use logo::*;
pub use title::*;

#[derive(Debug)]
pub enum GameScreen {
    LOGO = 0,
    TITLE,
    GAMEPLAY,
    ENDING,
}
