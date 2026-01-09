mod block;
mod blocks;
mod colors;
mod game;
mod grid;
mod position;
use raylib::prelude::*;

fn main() {
    let dark_blue: Color = Color {
        r: 44,
        g: 44,
        b: 127,
        a: 255,
    };
    let mut game = game::Game::new();
    let (mut rl, thread) = raylib::init().size(300, 600).title("Tetris-rs").build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        game.handle_input();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(dark_blue);
        game.draw(&mut d);
        //d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
