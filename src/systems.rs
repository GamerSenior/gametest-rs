extern crate specs;
extern crate sdl2;

use specs::{ Component, WriteStorage, VecStorage, System, Join};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }
}

pub struct MovimentSystem;

impl<'a> System<'a> for MovimentSystem {
    type SystemData = (WriteStorage<'a, Position>);

    fn run(&mut self, mut pos: Self::SystemData) {
        for pos in pos.join() {
            println!("{:?}", pos);
        }
    }
}
