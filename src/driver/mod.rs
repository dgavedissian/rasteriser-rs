mod glium;

pub use self::glium::KeyCode;
pub use self::glium::KeyState;

pub enum Colour {
    RGBA(u8, u8, u8, u8),
    HEX(u32)
}

impl Colour {
    fn to_raw(&self) -> (u8, u8, u8) {
        match *self {
            Colour::RGBA(r, g, b, _) => (r, g, b),
            Colour::HEX(h) => (
                ((h & 0xFF0000) >> 16) as u8,
                ((h & 0x00FF00) >> 8) as u8,
                (h & 0x0000FF) as u8
            )
        }
    }
}

pub trait Window {
    fn set_input_callback(&mut self, callback: Box<FnMut(KeyCode, KeyState)>);
    fn draw_pixel(&mut self, x: usize, y: usize, colour: &Colour);
    fn update(&mut self, clear_pixel_buffer: bool) -> bool;
}

pub trait Driver {
    fn create_window(&self, width: usize, height: usize, title: &str) -> Box<Window>;
}

pub enum DriverType {
    GL,
    Text
}

pub fn create(driver: DriverType) -> Box<Driver> {
    match driver {
        DriverType::GL => Box::new(glium::GliumDriver::new()),
        DriverType::Text => panic!("Text driver unimplemented")
    }
}