extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

struct Bar {
    width: f64,
    max_height: f64,
    curr_height: f64,
    color: Color,
}

impl Bar {
    fn rect_cooridinates(&self, x0: f64, y0: f64) -> [f64; 4] {
        let x1 = x0 + self.width;
        let y1 = y0 + self.max_height;
        let y0 = y1 - self.curr_height;

        rectangle::rectangle_by_corners(x0, y0, x1, y1)
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
            for bar in self.spectrogram.bars.iter() {
                rectangle(bar.color, bar.rect_cooridinates(x0, y0), c.transform, gl);

                // Move x coordinate over 10px
                x0 = x0 + bar.width + 10.0;
            }
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        for bar in self.spectrogram.bars.iter_mut() {
            let rand_percentage = rand::thread_rng().gen_range(40.0..=100.0);
            let ratio = rand_percentage / 100.0;
            bar.curr_height = bar.max_height * ratio;
        }
    }
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
            max_height: 300.0,
            curr_height: 300.0,
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
