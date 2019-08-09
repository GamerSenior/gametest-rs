extern crate specs;

#[macro_use]
extern crate specs_derive;
   
extern crate gl;

use specs::prelude::*;
use std::{ thread, time::Duration, };
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture, InitFlag};

mod systems;
pub mod render_gl;

use systems::{ MovimentSystem, Position };

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Testing Window", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Não foi possível inicializar os subsistemas de vídeo");

    let _gl_context = window.gl_create_context()?;
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

    // let mut canvas = window.into_canvas().build()
    //    .expect("Não foi possível criar o canvas");
    // let _texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(MovimentSystem, "sys_mov", &[]).build();
    dispatcher.setup(&mut world.res);

    world.create_entity().with(Position::new(1, 1)).build();

    dispatcher.dispatch(&mut world.res);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,  // tamanho dos dados em bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // ponteiro para os dados
            gl::STATIC_DRAW, // uso
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // faz unbind do buffer
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'main
                },
                _ => {}
            }
        }
        // Testing writing to canvas
        // canvas.clear();
        // canvas.set_draw_color(Color::RGB(0, 255, 0));
        // canvas.present();

        world.maintain();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
