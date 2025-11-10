use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::utils::workspaces::Workspace;
use crate::utils::ui::common::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

// Fixed I/O segment addresses
const IO_START: u32 = 0xE000;
const IO_SIZE: u32 = 0x1000;

#[derive(Clone, Default)]
pub struct IoDevicesState {
    pub input_buffer: String,
}

pub fn render_io_devices(
    frame: &mut Frame,
    area: Rect,
    workspace: &Workspace,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let output_buffer = if let Some(emulator) = &workspace.emulator {
        let mut buffer = Vec::new();
        for i in 0..(IO_SIZE / 2) {
            if let Ok(byte) = emulator.memory.read_u8(IO_START + IO_SIZE / 2 + i) {
                if byte == 0 {
                    break;
                }
                buffer.push(byte);
            }
        }
        String::from_utf8_lossy(&buffer).to_string()
    } else {
        String::new()
    };

    let output_paragraph = Paragraph::new(output_buffer)
        .block(Block::default().borders(Borders::ALL).title("Output"))
        .wrap(Wrap { trim: true });

    let input_paragraph = Paragraph::new(workspace.io_device.input_buffer.clone())
        .block(Block::default().borders(Borders::ALL).title("Input"));

    frame.render_widget(output_paragraph, chunks[0]);
    frame.render_widget(input_paragraph, chunks[1]);
}

pub fn handle_io_devices_input(
    key: KeyEvent,
    workspace: &mut Workspace,
    app_state: &mut AppState,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match key.code {
        KeyCode::Esc => {
            *app_state = AppState::Emulator;
            *handled = true;
        },
        KeyCode::Char(c) => {
            workspace.io_device.input_buffer.push(c);
            if let Some(emulator) = &mut workspace.emulator {
                let bytes = workspace.io_device.input_buffer.as_bytes();
                for (i, &byte) in bytes.iter().enumerate() {
                    emulator.memory.write_u8(IO_START + i as u32, byte).unwrap();
                }
                emulator.memory.write_u8(IO_START + bytes.len() as u32, 0).unwrap();
            }
            *handled = true;
        }
        KeyCode::Backspace => {
            workspace.io_device.input_buffer.pop();
            if let Some(emulator) = &mut workspace.emulator {
                let bytes = workspace.io_device.input_buffer.as_bytes();
                for (i, &byte) in bytes.iter().enumerate() {
                    emulator.memory.write_u8(IO_START + i as u32, byte).unwrap();
                }
                emulator.memory.write_u8(IO_START + bytes.len() as u32, 0).unwrap();
            }
            *handled = true;
        }
        _ => {}
    }
}
