mod block;
mod blocks;
mod colors;
mod game;
mod grid;
mod position;
use raylib::{
    ffi::{GetTime, MeasureTextEx, UpdateMusicStream},
    prelude::*,
};
use std::ffi::CString;

static mut LAST_UPDATE_TIME: f64 = 0.0;
fn main() {
    let mut game = game::Game::new();
    let (mut rl, thread) = raylib::init().size(500, 620).title("Tetris-rs").build();
    rl.set_target_fps(60);

    let font = rl
        .load_font_ex(&thread, "fonts/AGENCYB.TTF", 64, None)
        .expect("[  Main  ] Error loading font.");

    while !rl.window_should_close() {
        unsafe { UpdateMusicStream(game.music) };
        game.handle_input();
        if event_triggered(0.2) {
            game.move_block_down();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(colors::DARK_BLUE);
        d.draw_text_ex(
            &font,
            "Score",
            Vector2::new(365.0, 15.0),
            38.0,
            2.0,
            Color::WHITE,
        );
        d.draw_text_ex(
            &font,
            "Next",
            Vector2::new(370.0, 175.0),
            38.0,
            2.0,
            Color::WHITE,
        );
        if game.game_over {
            d.draw_text_ex(
                &font,
                "GAME OVER",
                Vector2::new(320.0, 450.0),
                38.0,
                2.0,
                Color::WHITE,
            );
        }
        d.draw_rectangle_rounded(
            Rectangle::new(320.0, 55.0, 170.0, 60.0),
            0.3,
            6,
            colors::LIGHT_BLUE,
        );
        let score_text = format!("{}", game.score);
        let c_text = CString::new(score_text.clone()).unwrap();
        let text_size = unsafe { ffi::MeasureTextEx(*font.as_ref(), c_text.as_ptr(), 38.0, 2.0) };
        d.draw_text_ex(
            &font,
            score_text.as_str(),
            Vector2::new(320.0 + (170.0 - text_size.x) / 2.0, 65.0),
            38.0,
            2.0,
            Color::WHITE,
        );

        d.draw_rectangle_rounded(
            Rectangle::new(320.0, 215.0, 170.0, 180.0),
            0.3,
            6,
            colors::LIGHT_BLUE,
        );

        game.draw(&mut d);

        //d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

fn event_triggered(interval: f64) -> bool {
    unsafe {
        let curr_time: f64 = GetTime();
        if curr_time - LAST_UPDATE_TIME >= interval {
            LAST_UPDATE_TIME = curr_time;
            return true;
        }
    };
    return false;
}
