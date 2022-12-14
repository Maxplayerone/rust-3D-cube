#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate gl;
use self::gl::types::*;

extern crate glutin;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

use image::EncodableLayout;

// Huh, so mod is sorta like #include...
mod common;
mod shader;
mod macros;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;

use shader::Shader;

extern crate image;
use image::GenericImageView;


extern crate cgmath;
use cgmath::{Matrix4, vec3, Deg, Rad, perspective};
use cgmath::prelude::*;

// settings 
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let ourShader: Shader;
    let mut vbo = 0;
    let mut vao = 0;
    let mut texture1 = 0;
    let mut texture2 = 0;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        // lol... looks like rust changed how it seeks files since
        // I last looked at this ages ago... now in the same directory as 
        // the cargo.toml file! 
        
        ourShader = Shader::new(
            "resources/shaders/vertshader.glsl",
            "resources/shaders/fragshader.glsl"
        );

        let vertices: [f32; 180] = [
             -0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5, -0.5,  0.0, 1.0,
              0.5,  0.5, -0.5,  0.0, 1.0,
              0.5,  0.5, -0.5,  0.0, 1.0,
             -0.5,  0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,

             -0.5, -0.5,  0.5,  0.0, 1.0,
              0.5, -0.5,  0.5,  0.0, 1.0,
              0.5,  0.5,  0.5,  0.0, 1.0,
              0.5,  0.5,  0.5,  0.0, 1.0,
             -0.5,  0.5,  0.5,  0.0, 1.0,
             -0.5, -0.5,  0.5,  0.0, 1.0,

             -0.5,  0.5,  0.5,  0.0, 1.0,
             -0.5,  0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5,  0.5,  0.0, 1.0,
             -0.5,  0.5,  0.5,  0.0, 1.0,

              0.5,  0.5,  0.5,  0.0, 1.0,
              0.5,  0.5, -0.5,  0.0, 1.0,
              0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5,  0.5,  0.0, 1.0,
              0.5,  0.5,  0.5,  0.0, 1.0,

             -0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5,  0.5,  0.0, 1.0,
              0.5, -0.5,  0.5,  0.0, 1.0,
             -0.5, -0.5,  0.5,  0.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,

             -0.5,  0.5, -0.5,  0.0, 1.0,
              0.5,  0.5, -0.5,  0.0, 1.0,
              0.5,  0.5,  0.5,  0.0, 1.0,
              0.5,  0.5,  0.5,  0.0, 1.0,
             -0.5,  0.5,  0.5,  0.0, 1.0,
             -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW);
        
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // texture coord attribute
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        
        /*
        // texture 1
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        //texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        
        let img = image::open(Path::new("resources/container.jpg")).unwrap().into_rgba8();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGBA as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0, 
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       img.as_bytes().as_ptr() as *const _);
                       
        gl::GenerateMipmap(gl::TEXTURE_2D);
        // texture 2
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        //texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        
        let img = image::open(Path::new("resources/awesomeface.png")).unwrap().into_rgba8();
        //let img = img.flipv();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGBA as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0, 
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       img.as_bytes().as_ptr() as *const _
                       );                  
        gl::GenerateMipmap(gl::TEXTURE_2D);
        
        ourShader.useProgram();
        ourShader.setInt(c_str!("texture1"), 0);
        ourShader.setInt(c_str!("texture2"), 1);
        */
    };
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        
                    
                    //gl::ActiveTexture(gl::TEXTURE0);
                    //gl::BindTexture(gl::TEXTURE_2D, texture1);
                    //gl::ActiveTexture(gl::TEXTURE1);
                    //gl::BindTexture(gl::TEXTURE_2D, texture2);
        
                    ourShader.useProgram();
        
                    // create transformations
                    // also, the equivalent of glm::rotate requires a normalized vector. Bit different
                    // from what I'm used to but it will adapt to service us :)
                    let model: Matrix4<f32>= Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), 
                                                                      Rad(32.0 as f32));
                    let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
                    let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                    // matrix uniform locations
                    let modelLoc = gl::GetUniformLocation(ourShader.ID, c_str!("model").as_ptr());
                    let viewLoc = gl::GetUniformLocation(ourShader.ID, c_str!("view").as_ptr());
                    // pass them to the shaders
                    gl::UniformMatrix4fv(modelLoc, 1, gl::FALSE, model.as_ptr());
                    gl::UniformMatrix4fv(viewLoc, 1, gl::FALSE, &view[0][0]);
                    // set projection matrix, probably better off outside but eh I'm just messing
                    // not trying to write efficiently
                    ourShader.setMat4(c_str!("projection"), &projection);
        
                    // render
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}