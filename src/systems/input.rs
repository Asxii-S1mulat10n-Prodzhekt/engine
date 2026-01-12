use bevy::prelude::*;
use bevy_ratatui::event::KeyMessage;
use ratatui::crossterm::event::KeyCode;

use crate::resources::events::MoveEvent;
pub fn input_system(
    mut messages: MessageReader<KeyMessage>,
    mut move_writer: MessageWriter<MoveEvent>,
    mut exit: MessageWriter<AppExit>,
) {
    for message in messages.read() {
        let (dx, dy, dz) = match message.code {
            KeyCode::Char('w') => (0.0, 1.0, 0.0),
            KeyCode::Char('s') => (0.0, -1.0, 0.0),
            KeyCode::Char('a') => (-1.0, 0.0, 0.0),
            KeyCode::Char('d') => (1.0, 0.0, 0.0),
            KeyCode::Char(' ') => (0.0, 0.0, 2.0),
            KeyCode::Esc => {
                exit.write_default();
                continue;
            }
            _ => continue,
        };

        move_writer.write(MoveEvent { dx, dy, dz });
    }
}
