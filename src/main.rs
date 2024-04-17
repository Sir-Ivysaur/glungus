extern crate sdl2;

// use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf;
use sdl2::image::LoadTexture;
use std::time::Duration;

fn lerp(start: u32, end: u32, t: f32) -> u32 {
    (start as f32 + t * (end as f32 - start as f32)) as u32
}
pub fn main() {
    let win_w = 500;
    let win_h = 500;
    
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video.window("glungus", win_w, win_h)
                 .position_centered()
                 .build()
                 .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    //IMPORTANT. KEEP GLUNGUS ALIVE.
    let texcreator = canvas.texture_creator();
    let glungus = texcreator.load_texture("./assets/glungus.png").unwrap();

    //IMPORTANT. CONTAIN GLUNGUS.
    let mut container = sdl2::rect::Rect::new(0,0,400,400);

    //IMPORTANT. DATA RESEARCH
    let mut clicked: u32 = 0;
    let mut t: f32;

    //IMPORTANT. DATA RENDERING
    let ttfctx = ttf::init().unwrap();
    let font = ttfctx.load_font("assets/Lexend-Bold.ttf", 128).unwrap();
    let text_container = sdl2::rect::Rect::new(0,0,win_w,50);

    let mut event_poll = ctx.event_pump().unwrap();
    let mut debounce = false;
    'game_loop: loop {
        canvas.clear();
        for event in event_poll.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {

                    if debounce == false {
                        clicked += 1;
                    }
                    debounce = true;
                    
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    debounce = false;
                }
                _ => {}
            }
        }

        //THIS IS SO FUCKING STUPID

        if debounce == true {
            t = 0.0;
            if t < 1.0 {
                t += 0.1; // Adjust this value to control the speed of the movement.
                container.set_width(lerp(container.width(), 300, t));
                container.set_height(lerp(container.height(), 300, t));
            }
        } else {
            t = 0.0;
            if t < 1.0 {
                t += 0.1; // Adjust this value to control the speed of the movement.
                container.set_width(lerp(container.width(), 380, t));
                container.set_height(lerp(container.height(), 380, t));
            }
        }

        container.set_x(((win_w - container.width()) / 2).try_into().unwrap());
        container.set_y(((win_h - container.height()) / 2).try_into().unwrap());

        // RENDER GLUNGUS.
        canvas.copy(&glungus, None, Some(container)).expect("GLUNGUS?? FAILED 2 RENDER?? WHAT");

        let text = font
            .render(&format!("you have pressed the glungus {} times", clicked))
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .unwrap();

        let texture = texcreator
            .create_texture_from_surface(&text)
            .unwrap();

        canvas.copy(&texture, None, Some(text_container)).expect("rendering font failed");
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
