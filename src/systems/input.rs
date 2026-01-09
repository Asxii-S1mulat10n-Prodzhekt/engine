use bevy::{app::AppExit, prelude::*};
use bevy_ratatui::event::KeyMessage;
use ratatui::crossterm::event::KeyCode;
pub fn input_system(mut messages: MessageReader<KeyMessage>, mut exit: MessageWriter<AppExit>) {
    for message in messages.read() {
        if let KeyCode::Char('q') = message.code {
            exit.write_default();
        }
    }
}
