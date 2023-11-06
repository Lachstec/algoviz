use raylib::prelude::*;

mod sorting;
mod visualizer;

fn main() {
    let ( mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("algoviz")
        .build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, Raylib", 12, 12, 20, Color::BLACK);
    }
}
