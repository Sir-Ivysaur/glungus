extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use std::time::Duration;

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video.window("glungus", 500, 500)
                 .position_centered()
                 .build()
                 .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();
    canvas.present();

    //IMPORTANT. KEEP GLUNGUS ALIVE.
    let texcreator = canvas.texture_creator();
    let glungus = texcreator.load_texture("./src/assets/glungus.png").unwrap();

    let mut event_poll = ctx.event_pump().unwrap();
    'game_loop: loop {
        canvas.clear();
        for event in event_poll.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                _ => {}
            }
        }

        // RENDER GLUNGUS.
        canvas.copy(&glungus, None, None).expect("Render failed");

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}