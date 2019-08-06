extern crate specs;

#[macro_use]
extern crate specs_derive;

use specs::prelude::*;
use std::{ thread, time::Duration };
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

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(MovimentSystem, "sys_mov", &[]).build();
    dispatcher.setup(&mut world.res);

    world.create_entity().with(Position::new(1, 1)).build();

    dispatcher.dispatch(&mut world.res);
    // read_synchronously();
    println!("Testing...");
    thread::sleep(Duration::from_millis(2000));

    Ok(())
}
