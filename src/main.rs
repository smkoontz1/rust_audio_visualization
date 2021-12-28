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

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

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

struct ChannelBars {
    left_bar: Bar,
    right_bar: Bar,
}

pub struct App {
    gl: GlGraphics,
    channel_bars: ChannelBars,
    // spectrogram: Spectrogram,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Y doesn't change
            let y0: f64 = 50.0;

            // Make bar references
            let right_bar = &self.channel_bars.right_bar;
            let left_bar = &self.channel_bars.left_bar;

            // Draw the left bar
            let left_x0: f64 = 20.0;
            rectangle(
                left_bar.color,
                left_bar.rect_cooridinates(left_x0, y0),
                c.transform,
                gl,
            );

            // Put 10px of space in between them
            let right_x0: f64 = left_x0 + left_bar.width + 10.0;

            // Draw the right bar
            rectangle(
                right_bar.color,
                right_bar.rect_cooridinates(right_x0, y0),
                c.transform,
                gl,
            );

            // Old code for spectro we probably want
            // // Draw bars
            // let mut x0: f64 = 20.0;
            // let y0: f64 = 50.0;
            // for bar in self.spectrogram.bars.iter() {
            //     rectangle(bar.color, bar.rect_cooridinates(x0, y0), c.transform, gl);

            //     // Move x coordinate over 10px
            //     x0 = x0 + bar.width + 10.0;
            // }
        })
    }

    fn update(&mut self) {
        // Some stuff
    }

    // fn update(&mut self, curr_left_value: &f32, curr_right_value: &f32) {
    //     let max_amplitude = 0.2;
    //     let left_ratio = f64::from(curr_left_value.abs() / max_amplitude);
    //     let right_ratio = f64::from(curr_right_value.abs() / max_amplitude);

    //     self.channel_bars.left_bar.curr_height = self.channel_bars.left_bar.max_height * left_ratio;
    //     self.channel_bars.right_bar.curr_height =
    //         self.channel_bars.right_bar.max_height * right_ratio;
    // }
}

fn main() -> anyhow::Result<()> {
    let opengl = OpenGL::V3_2;

    // Create the Glutin window.
    let mut window: Window = WindowSettings::new("Audio Visualization", [150, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let channel_bars = ChannelBars {
        left_bar: Bar {
            color: RED,
            width: 50.0,
            max_height: 300.0,
            curr_height: 300.0,
        },
        right_bar: Bar {
            color: RED,
            width: 50.0,
            max_height: 300.0,
            curr_height: 300.0,
        },
    };

    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        channel_bars,
    };

    // let host = cpal::host_from_id(cpal::HostId::Wasapi).expect("Failed to initialize Wasapi host.");
    // let device = host
    //     .default_output_device()
    //     .expect("No output device available.");
    // let supported_config = device.default_output_config().unwrap();
    // let config = supported_config.config();
    // let channels = config.channels as usize;
    // println!("Default output config: {:?}", config);

    // let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    // let mut left_channel_curr_value = 0.0;
    // let mut right_channel_curr_value = 0.0;

    // let stream = device
    //     .build_input_stream(
    //         &config,
    //         move |data: &[f32], _: &InputCallbackInfo| {
    //             for frame in data.chunks(channels) {
    //                 let mut channel = 0;

    //                 for sample in frame.iter() {
    //                     match channel {
    //                         0 => left_channel_curr_value = *sample,
    //                         1 => right_channel_curr_value = *sample,
    //                         _ => {
    //                             left_channel_curr_value = 0.0;
    //                             right_channel_curr_value = 0.0;
    //                         }
    //                     }

    //                     println!("Reading from channel: {} value: {}", channel, *sample);

    //                     // iterate the channel
    //                     channel = channel + 1;
    //                 }
    //             }
    //         },
    //         err_fn,
    //     )
    //     .expect("Error building input stream.");
    // stream.play()?;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            app.update();
            // app.update(&left_channel_curr_value, &right_channel_curr_value);
        }
    }

    // That is because it is needed up here, I see.
    thread::sleep(std::time::Duration::from_millis(10000));

    Ok(())

    // match config.sample_format() {
    //     SampleFormat::F32 => run_audio::<f32>(&device, &config.into()),
    //     SampleFormat::I16 => run_audio::<i16>(&device, &config.into()),
    //     SampleFormat::U16 => run_audio::<u16>(&device, &config.into()),
    // }
}

fn run_audio<T>(device: &Device, config: &StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_input_stream(
            config,
            move |data: &[T], _: &InputCallbackInfo| {
                for frame in data.chunks(channels) {
                    let mut channel = 0;
                    for sample in frame.iter() {
                        println!(
                            "Reading from channel: {} value: {}",
                            channel,
                            sample.to_f32()
                        );
                        channel = channel + 1;
                    }
                }
            },
            err_fn,
        )
        .expect("Error building input stream.");
    stream.play()?;

    // This doesn't seem to do anything anymore
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
