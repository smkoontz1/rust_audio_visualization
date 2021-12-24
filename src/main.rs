extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::types::Color;
use graphics::{rectangle, Rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Bar {
    width: f64,
    height: f64,
    color: Color,
}

impl Bar {
    fn rect_cooridinates(&self, x0: f64, y0: f64) -> [f64; 4] {
        rectangle::rectangle_by_corners(x0, y0, x0 + self.width, y0 + self.height)
    }
}

struct Spectrogram {
    bars: Vec<Bar>,
}

pub struct App {
    gl: GlGraphics,
    spectrogram: Spectrogram,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        // let bar = rectangle::rectangle_by_corners(20.0, 50.0, 120.0, 350.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Draw bars
            let mut x0: f64 = 20.0;
            let y0: f64 = 50.0;
            let bar_count = self.spectrogram.bars.len();
            for i in 0..bar_count {
                let bar = &self
                    .spectrogram
                    .bars
                    .get(i)
                    .expect("There was a problem getting the bar.");
                
                rectangle(bar.color, bar.rect_cooridinates(x0, y0), c.transform, gl);
                
                // Move x coordinate over 10px
                x0 = x0 + bar.width + 10.0;
            }
        })
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create the Glutin window.
    let mut window: Window = WindowSettings::new("Audio Visualization", [800, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a spectrogram.
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    // Make 7 red bars
    let mut bars = Vec::<Bar>::new();
    for _i in 1..=7 {
        bars.push(Bar {
            width: 100.0,
            height: 300.0,
            color: RED,
        })
    }

    let spectrogram = Spectrogram { bars };

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        spectrogram,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
