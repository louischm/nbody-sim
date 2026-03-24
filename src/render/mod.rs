use crate::physics::body::Body;
use macroquad::prelude::*;

pub fn draw_bodies(bodies: &[Body]) {
    let scale = 1e-9_f32;
    let center_x = screen_width() * 0.5;
    let center_y = screen_height() * 0.5;

    for body in bodies {
        let x= center_x + (body.position.x as f32) * scale;
        let y = center_y + (body.position.y as f32) * scale;

        let radius = (body.mass.abs().log10().max(1.0) as f32).clamp(2.0, 8.0);

         draw_circle(x, y, radius, body.to_render_color());
        draw_text(&body.name, x + 8.0, y - 8.0, 16.0, WHITE);
    }
}