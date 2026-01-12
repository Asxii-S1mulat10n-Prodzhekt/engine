use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::text::Text;

use crate::{components::position::Position, entities::Player};
pub fn draw_system(
    mut context: ResMut<RatatuiContext>,
    query: Query<&Position, With<Player>>,
) -> Result {
    context.draw(|frame| {
        for player in &query {
            let text = Text::raw(format!("{} ", player));
            frame.render_widget(text, frame.area());
        }
    })?;

    Ok(())
}
