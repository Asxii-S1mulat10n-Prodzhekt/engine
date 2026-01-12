use crate::{components::position::Position, entities::Player};
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::layout::Rect as RRect;
use ratatui::widgets::{Paragraph, Wrap};
pub fn draw_system(
    mut context: ResMut<RatatuiContext>,
    query: Query<&Position, With<Player>>,
) -> Result {
    context.draw(|frame| {
        for pos in &query {
            let (x, y, z) = pos.get_position_as_u16();
            let area = RRect {
                x,
                y,
                width: 1,
                height: 1,
            };

            let play = Paragraph::new("O");
            frame.render_widget(play, area);

            let area = RRect {
                x: 0,
                y: 0,
                width: 50,
                height: 1,
            };
            let overray = Paragraph::new(format!("{:?}", pos.get_position_as_u16()))
                .wrap(Wrap { trim: true });
            frame.render_widget(overray, area);
        }
    })?;

    Ok(())
}
