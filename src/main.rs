extern crate anyhow;
extern crate cpal;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, InputCallbackInfo, OutputCallbackInfo, SampleFormat, StreamConfig};
use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
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

fn main() -> anyhow::Result<()> {
    // let opengl = OpenGL::V3_2;

    // // Create the Glutin window.
    // let mut window: Window = WindowSettings::new("Audio Visualization", [800, 400])
    //     .graphics_api(opengl)
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();

    // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    // // Make 7 red bars
    // let mut bars = Vec::<Bar>::new();
    // for _i in 1..=7 {
    //     bars.push(Bar {
    //         width: 100.0,
    //         max_height: 300.0,
    //         curr_height: 300.0,
    //         color: RED,
    //     })
    // }

    // // Create a spectrogram.
    // let spectrogram = Spectrogram { bars };

    let host = cpal::host_from_id(cpal::HostId::Wasapi).expect("Failed to initialize Wasapi host.");
    let device = host
        .default_output_device()
        .expect("No output device available.");
    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    // Create a new game and run it.
    // let mut app = App {
    //     gl: GlGraphics::new(opengl),
    //     spectrogram,
    // };

    // let mut events = Events::new(EventSettings::new());
    // while let Some(e) = events.next(&mut window) {
    //     if let Some(args) = e.render_args() {
    //         app.render(&args);
    //     }

    //     if let Some(_args) = e.update_args() {
    //         app.update();
    //     }
    // }

    match config.sample_format() {
        SampleFormat::F32 => run_audio::<f32>(&device, &config.into()),
        SampleFormat::I16 => run_audio::<i16>(&device, &config.into()),
        SampleFormat::U16 => run_audio::<u16>(&device, &config.into()),
    }
}

fn run_audio<T>(device: &Device, config: &StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid
    // let mut sample_clock = 0f32;
    // let mut next_value = move || {
    //     sample_clock = (sample_clock + 1.0) % sample_rate;
    //     (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    // };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_input_stream(
            config,
            move |data: &[T], _: &InputCallbackInfo| {
                for frame in data.chunks(channels) {
                    let mut channel = 0;
                    for sample in frame.iter() {
                        println!("Reading from channel: {} value: {}", channel, sample.to_f32());
                        channel = channel + 1;
                    }
                }
            },
            err_fn,
        )
        .expect("Error building input stream.");
    stream.play()?;

    // let output_stream = device
    //     .build_output_stream(
    //         config,
    //         move |data: &mut [T], _: &OutputCallbackInfo| {
    //             write_data(data, channels, &mut next_value)
    //         },
    //         err_fn,
    //     )
    //     .expect("Error building output stream");
    // output_stream.play()?;

    thread::sleep(std::time::Duration::from_millis(5000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
