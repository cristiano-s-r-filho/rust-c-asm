//! # I/O Device Module
//!
//! This module defines the `IoDevice` struct, which simulates a basic
//! input/output device for the ARC CPU. It includes state for buttons
//! and buffers for text input and output.

/// Represents the state of various input buttons.
#[derive(Debug, Clone)]
pub struct ButtonState {
    /// State of the 'up' button.
    pub up: bool,
    /// State of the 'down' button.
    pub down: bool,
    /// State of the 'left' button.
    pub left: bool,
    /// State of the 'right' button.
    pub right: bool,
    /// State of the 'circle' button.
    pub circle: bool,
    /// State of the 'triangle' button.
    pub triangle: bool,
    /// State of the 'square' button.
    pub square: bool,
    /// State of the 'cross' button.
    pub cross: bool,
}

impl Default for ButtonState {
    /// Creates a new `ButtonState` with all buttons in their default (unpressed) state.
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonState {
    /// Creates a new `ButtonState` with all buttons unpressed.
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

/// Represents a simulated I/O device for the ARC CPU.
#[derive(Debug, Clone)]
pub struct IoDevice {
    /// Buffer for output text from the CPU.
    pub output_buffer: String,
    /// Buffer for input text to the CPU.
    pub input_buffer: String,
    /// The current state of the simulated buttons.
    pub button_state: ButtonState,
    /// A 2D vector representing the text matrix for display.
    pub text_matrix: Vec<Vec<u8>>,
    /// Flag indicating if an input interrupt is pending.
    pub input_interrupt_pending: bool,
    /// Flag indicating if an output interrupt is pending.
    pub output_interrupt_pending: bool,
}

impl Default for IoDevice {
    /// Creates a new `IoDevice` with empty buffers and default button states.
    fn default() -> Self {
        Self::new()
    }
}

impl IoDevice {
    /// Creates a new `IoDevice` with empty buffers and all buttons unpressed.
    pub fn new() -> Self {
        Self {
            output_buffer: String::new(),
            input_buffer: String::new(),
            button_state: ButtonState::new(),
            text_matrix: vec![vec![b' '; 80]; 25], // Default 25 rows, 80 columns
            input_interrupt_pending: false,
            output_interrupt_pending: false,
        }
    }
}
