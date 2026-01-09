use bevy::prelude::*;
use bevy_ratatui::event::KeyMessage;
use ratatui::crossterm::event::KeyCode;
pub fn input_system(mut messages: MessageReader<KeyMessage>, mut exit: MessageWriter<AppExit>) {
    for message in messages.read() {
        match message.code {
            KeyCode::Char('w') => {
                println!("w");
            }
            KeyCode::Char('a') => {
                println!("a");
            }
            KeyCode::Char('s') => {
                println!("s");
            }
            KeyCode::Char('d') => {
                println!("d");
            }
            KeyCode::Esc => {
                exit.write_default();
            }
            _ => {}
        }
    }
}
