use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

#[derive(Debug)]
pub struct Mouse {
    x: i16,
    y: i16,
    delta_x: i16,
    delta_y: i16,
}

impl Mouse {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            delta_x: 0,
            delta_y: 0,
        }
    }

    pub fn update_position(new_x: i16, new_y: i16) {
        let mut mouse = MOUSE.lock().unwrap();
        mouse.delta_x = new_x - mouse.x;
        mouse.delta_y = new_y - mouse.y;
        mouse.x = new_x;
        mouse.y = new_y;
    }

    pub fn get_position() -> (i16, i16) {
        let mouse = MOUSE.lock().unwrap();
        (mouse.x, mouse.y)
    }

    pub fn get_delta() -> (i16, i16) {
        let mouse = MOUSE.lock().unwrap();
        (mouse.delta_x, mouse.delta_y)
    }
}