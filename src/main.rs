extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;
use rodio::{OutputStream, Decoder, Sink};

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

    fn update(&mut self) {
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

    // let host = cpal::host_from_id(cpal::HostId::Wasapi).expect("Failed to initialize Wasapi host.");
    // let device = host
    //     .default_output_device()
    //     .expect("No output device available.");
    // let mut supported_configs_range = device
    //     .supported_output_configs()
    //     .expect("Error while querying configs.");
    // let supported_config = supported_configs_range
    //     .next()
    //     .expect("No supported config.")
    //     .with_max_sample_rate();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        spectrogram,
    };

    // Audio
    let (_stream, stream_handle) = OutputStream::try_default().expect("Error getting output stream.");
    let sink = Sink::try_new(&stream_handle).expect("Error getting sink.");

    let path = Path::new("C:\\_git\\rust_audio_visualization\\src\\assets\\music\\tell_it_to_my_heart.mp3");
    let file = BufReader::new(File::open(path).expect("Error opening music file."));
    let source = Decoder::new(file).expect("Error getting decoder.");
    sink.append(source);

    // This keeps the sink going, but we have a render loop, so we don't need this.
    // sink.sleep_until_end();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            app.update();
        }
    }
}
