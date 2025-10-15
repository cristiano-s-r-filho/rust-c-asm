#[derive(Debug, Clone)]
pub struct ButtonState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub circle: bool,
    pub triangle: bool,
    pub square: bool,
    pub cross: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            circle: false,
            triangle: false,
            square: false,
            cross: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IoDevice {
    pub output_buffer: String,
    pub input_buffer: String,
    pub button_state: ButtonState,
}

impl IoDevice {
    pub fn new() -> Self {
        Self {
            output_buffer: String::new(),
            input_buffer: String::new(),
            button_state: ButtonState::new(),
        }
    }
}
