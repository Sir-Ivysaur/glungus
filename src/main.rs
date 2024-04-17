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

fn lerp(start: i32, end: i32, t: f32) -> u32 {
    (start as f32 + t * (end - start) as f32) as u32
}
pub fn main() {
    let window_width = 500;
    let window_height = 500;

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
        .load_texture_bytes(include_bytes!("glungus.png"))
        .unwrap();

    // IMPORTANT. CONTAIN GLUNGUS.
    let mut container = Rect::new(0, 0, 400, 400);

    // IMPORTANT. DATA RESEARCH
    let mut clicked: u32 = 0;
    let mut t: f32;

    // IMPORTANT. DATA RENDERING
    let ttfctx = ttf::init().unwrap();
    let font_bytes = RWops::from_bytes(include_bytes!("Lexend-Bold.ttf"))
        .expect("can't create a RWops from Lexend-Bold.ttf");
    let font = ttfctx.load_font_from_rwops(font_bytes, 128).unwrap();
    let text_container = sdl2::rect::Rect::new(0, 0, window_width, 50);

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
                container.set_width(lerp(container.width() as i32, 300, t));
                container.set_height(lerp(container.height() as i32, 300, t));
            }
        } else {
            t = 0.0;
            if t < 1.0 {
                t += 0.1; // Adjust this value to control the speed of the movement.
                container.set_width(lerp(container.width() as i32, 380, t));
                container.set_height(lerp(container.height() as i32, 380, t));
            }
        }

        container.set_x(((window_width - container.width()) / 2).try_into().unwrap());
        container.set_y(((window_height - container.height()) / 2).try_into().unwrap());

        // RENDER GLUNGUS.
        canvas
            .copy(&glungus, None, Some(container))
            .expect("GLUNGUS?? FAILED 2 RENDER?? WHAT");

        let text = font
            .render(&format!("you have pressed the glungus {} times", clicked))
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
