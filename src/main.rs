extern crate specs;

#[macro_use]
extern crate specs_derive;

use specs::prelude::*;
use std::{ thread, time::Duration };
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture, InitFlag};

mod systems;
use systems::{ MovimentSystem, Position };

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Testing Window", 800, 600)
        .position_centered()
        .build()
        .expect("Não foi possível inicializar os subsistemas de vídeo");

    let mut canvas = window.into_canvas().build()
        .expect("Não foi possível criar o canvas");
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(MovimentSystem, "sys_mov", &[]).build();
    dispatcher.setup(&mut world.res);

    world.create_entity().with(Position::new(1, 1)).build();

    dispatcher.dispatch(&mut world.res);
    // read_synchronously();
   
    let mut i = 0;

    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        // Testing writing to canvas
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.present();

        world.maintain();

        i = (i + 1) % 255;
        println!("{}", i);
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
