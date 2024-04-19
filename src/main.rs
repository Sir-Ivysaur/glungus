// use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::ttf;
use std::thread;
use std::time::Duration;

use rand::prelude::*;

fn lerp(start: i32, end: i32, t: f32) -> i32 {
    (start as f32 + t * (end - start) as f32) as i32
}
pub fn main() {
    let window_width = 640;
    let window_height = 480;

    let glungus_clickwidth = 250;
    let glungus_idlewidth = 300;

    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video
        .window("glungus", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    // IMPORTANT. KEEP GLUNGUS ALIVE.
    let texture_creator = canvas.texture_creator();
    let glungus = texture_creator
        .load_texture_bytes(include_bytes!("assets/glungus.png"))
        .unwrap();

    // IMPORTANT. CONTAIN GLUNGUS.
    let mut container = Rect::new(0, 0, glungus_idlewidth as u32, glungus_idlewidth as u32);

    // IMPORTANT. DATA RESEARCH
    let mut clicked: u32 = 0;
    let mut t: f32;

    let mut glangle: f64 = 0.0; // BEST NAME EVER
    let mut target_angle: f64 = 0.0;

    let mut rng = thread_rng();

    // IMPORTANT. DATA RENDERING
    let ttfctx = ttf::init().unwrap();
    let font_bytes = RWops::from_bytes(include_bytes!("assets/Lexend-Bold.ttf"))
        .expect("can't create a RWops from Lexend-Bold.ttf");
    let font = ttfctx.load_font_from_rwops(font_bytes, 128).unwrap();
    let text_container = sdl2::rect::Rect::new(10, 5, 170, 50);

    let mut event_poll = ctx.event_pump().unwrap();
    let mut debounce = false;
    'game_loop: loop {
        canvas.clear();
        for event in event_poll.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if !debounce {
                        clicked += 1;
                        target_angle = rng.gen_range(-35..35) as f64;
                    }
                    debounce = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    debounce = false;
                }
                _ => {}
            }
        }

        // THIS IS SO FUCKING STUPID

        if debounce {
            t = 0.0;
            if t < 1.0 {
                t += 0.1; // Adjust this value to control the speed of the movement.
                container.set_width(lerp(container.width() as i32, glungus_clickwidth, t) as u32);
                container.set_height(lerp(container.height() as i32, glungus_clickwidth, t) as u32);
                if clicked % 20 == 0 {
                    glangle = 360.0;
                } else {
                    glangle = lerp(glangle as i32, target_angle as i32, t) as f64;
                }
            }
        } else {
            t = 0.0;
            if t < 1.0 {
                t += 0.1; // Adjust this value to control the speed of the movement.
                container.set_width(lerp(container.width() as i32, glungus_idlewidth, t) as u32);
                container.set_height(lerp(container.height() as i32, glungus_idlewidth, t) as u32);
                glangle = lerp(glangle as i32, 0, t) as f64;
            }
        }

        container.set_x(
            ((window_width - container.width()) / 2)
                .try_into()
                .unwrap()
        );

        container.set_y(
            ((window_height - container.height()) / 2)
                .try_into()
                .unwrap(),
        );

        // RENDER GLUNGUS.
        canvas
            .copy_ex(&glungus, None, Some(container), glangle, None, false, false)
            .expect("GLUNGUS?? FAILED 2 RENDER?? WHAT");

        let text = font
            .render(&format!("-> {} times", clicked))
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();

        let texture = texture_creator.create_texture_from_surface(&text).unwrap();

        canvas
            .copy(&texture, None, Some(text_container))
            .expect("rendering font failed");
        canvas.present();
        thread::sleep(Duration::from_millis(17));
    }
}
