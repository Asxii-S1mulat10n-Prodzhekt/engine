use crate::{components::position::Position, entities::Player};
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::Frame;
use ratatui::layout::Rect as RRect;
use ratatui::style::{Color as RColor, Style};
use ratatui::widgets::{Paragraph, Wrap};
fn draw_wall(
    frame: &mut Frame,
    screen_weight: i16,
    screen_height: i16,
    color: RColor,
    x1: i16,
    x2: i16,
    y1: i16,
    y2: i16,
    y3: i16,
    y4: i16,
) {
    let x1 = if x1 < 1 { 1 } else { x1 };
    let x2 = if x2 < 1 { 1 } else { x2 };
    let x1 = if x1 > screen_weight {
        screen_weight - 1
    } else {
        x1
    };
    let x2 = if x2 > screen_weight {
        screen_weight - 1
    } else {
        x2
    };
    let dyb = y2 - y1;
    let dyt = y4 - y3;
    let dx = if x2 - x1 == 0 { 1 } else { x2 - x1 };
    let xs = x1;
    let overray = Paragraph::new("█").style(Style::default().fg(color).bg(RColor::Black));
    for x in x1..x2 {
        let yb = dyb * (((x as f32) - (xs as f32) + 0.5) as i16) / dx + y1;
        let yt = dyt * (((x as f32) - (xs as f32) + 0.5) as i16) / dx + y3;
        let y_start = yb.max(0);
        let y_end = yt.min(screen_height - 1);
        if x >= screen_weight {
            continue;
        }

        for y in y_start..y_end {
            let area = RRect {
                x: x as u16,
                y: y as u16,
                width: 1,
                height: 1,
            };
            frame.render_widget(&overray, area);
        }
    }
}

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
            let x1: i16 = 40 - x as i16;
            let y1: i16 = 40 - y as i16;
            let z1: i16 = 20;
            let wx1: i16 = x1 * 20 / y1 + half_screen_width as i16;
            let wy1: i16 = z1 * 20 / y1 + half_screen_height as i16;

            let x2: i16 = 40 - x as i16;
            let y2: i16 = 30 - y as i16;
            let z2: i16 = 20;
            let wx2: i16 = x2 * 20 / y2 + half_screen_width as i16;
            let wy2: i16 = z2 * 20 / y2 + half_screen_height as i16;

            let x3: i16 = 40 - x as i16;
            let y3: i16 = 40 - y as i16;
            let z3: i16 = -20;
            let wx3: i16 = x3 * 20 / y3 + half_screen_width as i16;
            let wy3: i16 = z3 * 20 / y3 + half_screen_height as i16;

            let x4: i16 = 40 - x as i16;
            let y4: i16 = 30 - y as i16;
            let z4: i16 = -20;
            let wx4: i16 = x4 * 20 / y4 + half_screen_width as i16;
            let wy4: i16 = z4 * 20 / y4 + half_screen_height as i16;
            draw_wall(
                frame,
                screen_width as i16,
                screen_height as i16,
                RColor::Red,
                wx1,
                wx2,
                wy3,
                wy4,
                wy1,
                wy2,
            );

            let x1: i16 = -20 - x as i16;
            let y1: i16 = 40 - y as i16;
            let z1: i16 = 20;
            let wx1: i16 = x1 * 20 / y1 + half_screen_width as i16;
            let wy1: i16 = z1 * 20 / y1 + half_screen_height as i16;

            let x2: i16 = 40 - x as i16;
            let y2: i16 = 40 - y as i16;
            let z2: i16 = 20;
            let wx2: i16 = x2 * 20 / y2 + half_screen_width as i16;
            let wy2: i16 = z2 * 20 / y2 + half_screen_height as i16;

            let x3: i16 = -20 - x as i16;
            let y3: i16 = 40 - y as i16;
            let z3: i16 = -20;
            let wx3: i16 = x3 * 20 / y3 + half_screen_width as i16;
            let wy3: i16 = z3 * 20 / y3 + half_screen_height as i16;

            let x4: i16 = 40 - x as i16;
            let y4: i16 = 40 - y as i16;
            let z4: i16 = -20;
            let wx4: i16 = x4 * 20 / y4 + half_screen_width as i16;
            let wy4: i16 = z4 * 20 / y4 + half_screen_height as i16;
            draw_wall(
                frame,
                screen_width as i16,
                screen_height as i16,
                RColor::LightCyan,
                wx1,
                wx2,
                wy3,
                wy4,
                wy1,
                wy2,
            );
            // draw_dot(frame, wx1, wy1);
            // draw_star(frame, wx2, wy2);
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
