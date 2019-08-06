extern crate specs;

#[macro_use]
extern crate specs_derive;

use specs::prelude::*;
use std::{ thread, time::Duration };

mod systems;
use systems::{ MovimentSystem, Position };

fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(MovimentSystem, "sys_mov", &[]).build();
    dispatcher.setup(&mut world.res);

    world.create_entity().with(Position::new(1, 1)).build();

    dispatcher.dispatch(&mut world.res);
    // read_synchronously();
    println!("Testing...");
    thread::sleep(Duration::from_millis(2000));
}
