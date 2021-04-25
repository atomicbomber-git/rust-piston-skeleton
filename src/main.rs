use std::f64::consts::PI;

use graphics::math::rotate_radians;
use piston_window::*;
use piston_window::types::Color;
use std::time::{UNIX_EPOCH, SystemTime};

const WINDOW_TITLE: &str = "Skeleton Piston Project";
const WINDOW_WIDTH_PIXELS: f64 = 640.0;
const WINDOW_HEIGHT_PIXELS: f64 = 480.0;
const COLOR_BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const COLOR_RED: Color = [200.0, 0.0, 0.0, 1.0];

const FRAME_PER_SECONDS: u128 = 60;
const MILLIS_PER_FRAME: u128 = (1000.0 / FRAME_PER_SECONDS as f64) as u128;


fn main() {
    let mut window: PistonWindow<> =
        WindowSettings::new(
            WINDOW_TITLE,
            [WINDOW_WIDTH_PIXELS, WINDOW_HEIGHT_PIXELS], )
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut rotation_degree = 0.0;
    let mut previous_update = UNIX_EPOCH;

    // 1 update step
    let step = |rotation_degree: &mut f64| {
        *rotation_degree = *rotation_degree + 1.0;
        if *rotation_degree > 360.0 {
            *rotation_degree = 0.0
        }
    };

    // MAIN LOOP
    while let Some(event) = window.next() {
        // This part of code ensures that the program always runs at the predetermined amount of FPS rate, e.g. 60
        if previous_update.elapsed().map(|d| d.as_millis()).unwrap_or(0) > MILLIS_PER_FRAME {
            let step_start = SystemTime::now();

            // Update by 1 step
            step(&mut rotation_degree);


            println!("Step took: {}ms", step_start.elapsed().map(|d| d.as_micros()).unwrap_or(0) as f32 / 1000.0);
            previous_update = SystemTime::now();
        }

        window.draw_2d(&event, |context, graphics, _device| {
            // DRAW HERE
            let context = context
                .trans(WINDOW_WIDTH_PIXELS / 2.0, WINDOW_HEIGHT_PIXELS / 2.0)
                .rot_deg(rotation_degree);

            // CLEAR SCREEN
            clear(COLOR_BLACK, graphics);

            // DRAW RECTANGLE
            Rectangle
            ::new(COLOR_RED)
                .draw(
                    [-50.0, -50.0, 50.0, 50.0],
                    &Default::default(),
                    context.transform,
                    graphics,
                );
        });
    }
}
