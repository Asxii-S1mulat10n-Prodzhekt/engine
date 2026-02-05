use std::i16::MAX;

use crate::resources::map::Obstacles;
use crate::{components::position::Position, entities::Player};
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ndarray::Axis;
use ratatui::Frame;
use ratatui::layout::Rect as RRect;
use ratatui::style::{Color as RColor, Style};
use ratatui::widgets::{Paragraph, Wrap};

fn draw_dot(frame: &mut Frame, x1: i16, y1: i16) {
    let area = RRect {
        x: x1 as u16,
        y: y1 as u16,
        width: 1,
        height: 1,
    };
    let overray = Paragraph::new("0");
    frame.render_widget(overray, area);
}

fn draw_star(frame: &mut Frame, x1: i16, y1: i16) {
    let area = RRect {
        x: x1 as u16,
        y: y1 as u16,
        width: 1,
        height: 1,
    };
    let overray = Paragraph::new("*");
    frame.render_widget(overray, area);
}
pub fn draw_system(
    mut context: ResMut<RatatuiContext>,
    obstacles: Res<Obstacles>,
    query: Query<&Position, With<Player>>,
) -> Result {
    context.draw(|frame| {
        for pos in &query {
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
            // let obs = obstacles.get_positions();
            // for ob in obs.axis_iter(Axis(1)) {
            //     for i in ob.axis_iter(Axis(0)) {
            //         draw_star(frame, i[0] as i16, i[1] as i16);
            //     }
            // }
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
