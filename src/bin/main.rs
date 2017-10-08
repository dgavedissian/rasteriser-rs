extern crate rasteriser;

fn main() {
    use rasteriser::driver::{Colour, DriverType, KeyCode, KeyState};

    let size = (640, 480);
    let driver = rasteriser::driver::create(DriverType::GL);
    let mut window = driver.create_window(size.0, size.1, "Hello World");
    window.set_input_callback(Box::new(|key: KeyCode, state: KeyState | {
        if state == KeyState::Pressed {
            println!("Key pressed: {:?}", key);
        } else {
            println!("Key released: {:?}", key);
        }
    }));
    while window.update(true) {
        for i in 50..350 {
            window.draw_pixel(i, 50, &Colour::RGBA(255, 50, 50, 255));
        }
    }
}