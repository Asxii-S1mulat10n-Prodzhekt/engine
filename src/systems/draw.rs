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
            let screen_height = frame.area().height;
            let screen_width = frame.area().width;

            let half_screen_height = frame.area().height / 2;
            let half_screen_width = frame.area().width / 2;
            let area = RRect {
                x: 0,
                y: 2,
                width: 50,
                height: 2,
            };
            let screen_overray = Paragraph::new(format!(
                "screen_height :{}\nscreen_width :{}",
                screen_height, screen_width
            ));
            frame.render_widget(screen_overray, area);
            let x1 = 40 - x;
            let y1 = 30 - y;
            let z1 = 1;
            let wx1 = x1 * 20 / y1 + half_screen_width;
            let wy1 = z1 * 20 / y1 + half_screen_height;

            let p1 = Paragraph::new("O");
            let area = RRect {
                x: wx1,
                y: wy1,
                width: 1,
                height: 1,
            };
            frame.render_widget(&p1, area);

            let x2 = 40 - x;
            let y2 = 50 - y;
            let z2 = 1;
            let wx2 = x2 * 20 / y2 + half_screen_width;
            let wy2 = z2 * 20 / y2 + half_screen_height;

            let area = RRect {
                x: wx2,
                y: wy2,
                width: 1,
                height: 1,
            };
            frame.render_widget(p1, area);

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
