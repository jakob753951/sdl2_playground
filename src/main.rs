extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

fn main() {
    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem.window("Pixel Manipulation", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    // Create a renderer
    let mut canvas = window.into_canvas().build().unwrap();

    // Get the texture creator
    let texture_creator = canvas.texture_creator();

    // Create a texture to manipulate pixels
    let mut texture = texture_creator.create_texture_target(sdl2::pixels::PixelFormatEnum::ARGB8888, 640, 480).unwrap();

    draw_texture(&mut texture);

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Update the texture with manipulated pixels
        // (Here you would manipulate the pixel array of the texture)

        // Render the texture
        canvas.copy(&texture, None, None).unwrap();

        // Update the canvas
        canvas.present();
    }
}

fn draw_texture(texture: &mut Texture) {
    for x in 0..640 {
        // set top pixels

    }
}