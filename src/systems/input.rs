use crate::resources::keyboard::MovementMessage;
use bevy::prelude::*;
use bevy_ratatui::event::KeyMessage;
use ratatui::crossterm::event::KeyCode;
pub fn input_system(
    mut messages: MessageReader<KeyMessage>,
    mut move_message: MessageWriter<MovementMessage>,
    mut exit: MessageWriter<AppExit>,
) {
    for message in messages.read() {
        match message.code {
            KeyCode::Char('w') => {
                move_message.write(MovementMessage::Forward);
            }
            KeyCode::Char('a') => {
                move_message.write(MovementMessage::Left);
            }
            KeyCode::Char('s') => {
                move_message.write(MovementMessage::Right);
            }
            KeyCode::Char('d') => {
                move_message.write(MovementMessage::Backward);
            }
            KeyCode::Esc => {
                exit.write_default();
            }
            _ => {}
        }
    }
}
