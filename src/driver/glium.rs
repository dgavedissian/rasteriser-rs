extern crate glium;

use self::glium::Surface;
use self::glium::glutin;
use driver::{Colour, Window, Driver};

pub use self::glium::glutin::VirtualKeyCode as KeyCode;
pub use self::glium::glutin::ElementState as KeyState;

// Driver

pub struct GliumDriver;

impl GliumDriver {
    pub fn new() -> GliumDriver {
        GliumDriver {}
    }
}

impl Driver for GliumDriver {
    fn create_window(&self, width: usize, height: usize, title: &str) -> Box<Window> {
        Box::new(GliumWindow::new(width, height, title))
    }
}

// Window

struct GliumWindow {
    width: usize,
    height: usize,
    window: glium::Display,
    events_loop: glutin::EventsLoop,
    key_callback: Box<FnMut(KeyCode, KeyState)>,
    display_texture: glium::Texture2d,
    pixel_buffer: Vec<u8>
}

#[inline]
fn set_pixel(pixels: &mut Vec<u8>, x: usize, y: usize, width: usize, height: usize, colour: (u8, u8, u8)) {
    if x >= width || y >= height {
        return;
    }
    let pixel_offset = (y * width + x) * 3;
    pixels[pixel_offset] = colour.0;
    pixels[pixel_offset + 1] = colour.1;
    pixels[pixel_offset + 2] = colour.2;
}

#[inline]
fn empty_pixels(width: usize, height: usize) -> Vec<u8> {
    vec![0; width * height * 3]
}

#[inline]
fn clear_pixels(width: usize, height: usize, colour: (u8, u8, u8)) -> Vec<u8> {
    let mut pixels = empty_pixels(width, height);
    for x in 0..width {
        for y in 0..height {
            set_pixel(&mut pixels, x, y, width, height, colour);
        }
    }
    pixels
}

impl GliumWindow {
    fn new(width: usize, height: usize, title: &str) -> GliumWindow {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(width as u32, height as u32)
            .with_title(title);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let surface_texture = glium::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            width as u32, height as u32
        ).unwrap();

        GliumWindow {
            width: width,
            height: height,
            window: display,
            events_loop: events_loop,
            key_callback: Box::new(|_: KeyCode, _: KeyState| {}),
            display_texture: surface_texture,
            pixel_buffer: empty_pixels(width, height)
        }
    }
}


impl Window for GliumWindow {
    fn set_input_callback(&mut self, callback: Box<FnMut(KeyCode, KeyState)>) {
        self.key_callback = callback;
    }

    fn draw_pixel(&mut self, x: usize, y: usize, colour: &Colour) {
        // y-coordinate inversion required because OpenGL specifies pixels from the bottom-left.
        set_pixel(&mut self.pixel_buffer, x, self.height - y - 1, self.width, self.height, colour.to_raw());
    }

    fn update(&mut self, clear_pixel_buffer: bool) -> bool {
        // Write pixels to the texture.
        let rect = glium::Rect {
            left: 0,
            bottom: 0,
            width: self.width as u32,
            height: self.height as u32
        };
        let raw_image_data = glium::texture::RawImage2d::from_raw_rgb(
            self.pixel_buffer.clone(),
            (rect.width, rect.height));
        self.display_texture.write(rect, raw_image_data);

        // Get pixel data.
        if clear_pixel_buffer {
            self.pixel_buffer = empty_pixels(self.width, self.height);
        }

        // Blit the texture to the backbuffer.
        let target = self.window.draw();
        self.display_texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        target.finish().unwrap();

        // Message pump.
        let mut running = true;
        let key_callback = &mut self.key_callback;
        self.events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{event, ..} => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput{input, ..} => {
                        if let Some(keycode) = input.virtual_keycode {
                            key_callback(keycode, input.state);
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        return running;
    }
}