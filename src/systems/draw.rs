use crate::{components::position::Position, entities::Player};
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::Frame;
use ratatui::layout::Rect as RRect;
use ratatui::style::{Color as RColor, Style};
use ratatui::widgets::{Paragraph, Wrap};
fn clip_wall(x1: &mut i16, y1: &mut i16, z1: &mut i16, x2: i16, y2: i16, z2: i16) {
    let da = *y1;
    let db = y2;
    let d = da - db;
    let d = if d == 0 { 1.0 } else { (da - db) as f32 };
    let s = da as f32 / d;
    *x1 = (*x1 as f32 + s * (x2 - (*x1)) as f32) as i16;
    *y1 = (*y1 as f32 + s * (y2 - (*y1)) as f32) as i16;
    if *y1 == 0 {
        *y1 = 1;
    }
    *z1 = (*z1 as f32 + s * (z2 - (*z1)) as f32) as i16;
}

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
    let dyb = if dyb == 0 { 1 } else { dyb };
    let dyt = if dyt == 0 { 1 } else { dyt };
    let dx = if x2 - x1 == 0 { 1 } else { x2 - x1 };
    let xs = x1;
    let overray = Paragraph::new("█").style(Style::default().fg(color).bg(RColor::Black));
    for x in x1..x2 {
        let t = (x as f64 - xs as f64 + 0.5) / dx as f64;
        let yb = (dyb as f64 * t + y1 as f64)
            .round()
            .clamp(i16::MIN as f64, i16::MAX as f64) as i16;

        let yt = (dyt as f64 * t + y3 as f64)
            .round()
            .clamp(i16::MIN as f64, i16::MAX as f64) as i16;
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
            let mut x1: i16 = 40 - x as i16;
            let mut y1: i16 = 40 - y as i16;
            let mut z1: i16 = 10 + z as i16;
            let mut x2: i16 = 40 - x as i16;
            let mut y2: i16 = 30 - y as i16;
            let mut z2: i16 = 20 + z as i16;
            let mut x3: i16 = 40 - x as i16;
            let mut y3: i16 = 40 - y as i16;
            let mut z3: i16 = -10 + z as i16;
            let mut x4: i16 = 40 - x as i16;
            let mut y4: i16 = 30 - y as i16;
            let mut z4: i16 = -20 + z as i16;
            if x1 < 1 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
                std::mem::swap(&mut z1, &mut z2);
                std::mem::swap(&mut x3, &mut x4);
                std::mem::swap(&mut y3, &mut y4);
                std::mem::swap(&mut z3, &mut z4);
            }
            if y1 == 0 && y2 < 0 {
                return;
            }

            if y1 < 1 {
                clip_wall(&mut x1, &mut y1, &mut z1, x2, y2, z2);
                clip_wall(&mut x3, &mut y3, &mut z3, x4, y4, z4);
            }

            if y2 < 1 {
                clip_wall(&mut x2, &mut y2, &mut z2, x1, y1, z1);
                clip_wall(&mut x4, &mut y4, &mut z4, x3, y3, z3);
            }
            let mut wx1: i16 = x1 * 20 / y1 + half_screen_width as i16;
            let mut wy1: i16 = z1 * 20 / y1 + half_screen_height as i16;

            let mut wx2: i16 = x2 * 20 / y2 + half_screen_width as i16;
            let mut wy2: i16 = z2 * 20 / y2 + half_screen_height as i16;
            let mut wx3: i16 = x3 * 20 / y3 + half_screen_width as i16;
            let mut wy3: i16 = z3 * 20 / y3 + half_screen_height as i16;

            let mut wx4: i16 = x4 * 20 / y4 + half_screen_width as i16;
            let mut wy4: i16 = z4 * 20 / y4 + half_screen_height as i16;
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

            // let x1: i16 = -20 - x as i16; let y1: i16 = 40 - y as i16;
            // let z1: i16 = 20 + z as i16;
            // let wx1: i16 = x1 * 20 / y1 + half_screen_width as i16;
            // let wy1: i16 = z1 * 20 / y1 + half_screen_height as i16;
            //
            // let x2: i16 = 40 - x as i16;
            // let y2: i16 = 40 - y as i16;
            // let z2: i16 = 20 + z as i16;
            // let wx2: i16 = x2 * 20 / y2 + half_screen_width as i16;
            // let wy2: i16 = z2 * 20 / y2 + half_screen_height as i16;
            //
            // let x3: i16 = -20 - x as i16;
            // let y3: i16 = 40 - y as i16;
            // let z3: i16 = -20 + z as i16;
            // let wx3: i16 = x3 * 20 / y3 + half_screen_width as i16;
            // let wy3: i16 = z3 * 20 / y3 + half_screen_height as i16;
            //
            // let x4: i16 = 40 - x as i16;
            // let y4: i16 = 40 - y as i16;
            // let z4: i16 = -20 + z as i16;
            // let wx4: i16 = x4 * 20 / y4 + half_screen_width as i16;
            // let wy4: i16 = z4 * 20 / y4 + half_screen_height as i16;
            // draw_wall(
            //     frame,
            //     screen_width as i16,
            //     screen_height as i16,
            //     RColor::LightCyan,
            //     wx1,
            //     wx2,
            //     wy3,
            //     wy4,
            //     wy1,
            //     wy2,
            // );
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
