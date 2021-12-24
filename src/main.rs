extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.0];

        let bar = rectangle::centered([0.0, 0.0, 50.0, 150.0]);
        let x = 20.0;
        let y = 50.0;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y);

            rectangle(RED, bar, transform, gl);
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
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

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl)
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
