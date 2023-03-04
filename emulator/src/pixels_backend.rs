const PIXEL_SCALE: f64 = 8.0;

use log::{info, error};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder, platform::unix::EventLoopBuilderExtUnix,
};
use winit_input_helper::WinitInputHelper;
use std::{sync::mpsc::{Sender, Receiver, channel}, thread::JoinHandle};

use crate::{Frame, Blinkmojt};

pub struct PixelsFrame {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

enum Command {
    Stop, Draw(Vec<u8>)
}

pub struct PixelsBlinkmojt {
    width: u32,
    height: u32,
    sender: Sender<Command>,
    thread: JoinHandle<()>,
}

impl PixelsFrame {
    fn new(width: u32, height: u32) -> PixelsFrame {
        PixelsFrame {
            width, height, pixels: vec![0; (width * height * 4) as usize]
        }
    }
}

impl Frame for PixelsFrame {
    fn set_pixel(&mut self, position: (i32, i32), color: u32) {
        let (x, y) = position;
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        let r = (color >> 24) & 0xff;
        let g = (color >> 16) & 0xff;
        let b = (color >> 8) & 0xff;
        let a = (color >> 8) & 0xff;

        self.pixels[y * 4 * (self.width as usize) + x * 4] = r as u8;
        self.pixels[y * 4 * (self.width as usize) + x * 4 + 1] = g as u8;
        self.pixels[y * 4 * (self.width as usize) + x * 4 + 2] = b as u8;
        self.pixels[y * 4 * (self.width as usize) + x * 4 + 3] = a as u8;
    }
}

pub fn open(width: u32, height: u32) -> PixelsBlinkmojt {
    env_logger::init();
    info!("opening blinkmojt");

    let (sender, receiver) = channel::<Command>();

    let thread = std::thread::spawn(move || {
        run(width, height, receiver);
    });

    PixelsBlinkmojt {
        width, height,
        sender, thread,
    }
}

impl Blinkmojt for PixelsBlinkmojt {
    type Frame = PixelsFrame;

    fn close(self) {
        info!("sending stop command");
        self.sender.send(Command::Stop).unwrap();
        info!("waiting for stop");
        self.thread.join().unwrap();
        info!("dead");
    }

    fn get_frame(&mut self) -> Self::Frame {
        PixelsFrame::new(self.width, self.height)
    }

    fn draw_frame(&mut self, frame: Self::Frame) {
        self.sender.send(Command::Draw(frame.pixels)).unwrap();
    }
}

fn run(width: u32, height: u32, receiver: Receiver<Command>) {
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        let scaled_size = LogicalSize::new(width as f64 * PIXEL_SCALE, height as f64 * PIXEL_SCALE);
        WindowBuilder::new()
            .with_title("Particles")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };
    pixels.set_clear_color(pixels::wgpu::Color::BLACK);
    
    let mut current_frame = vec![0 as u8; (width * height * 4) as usize];

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            {
                let pixels = pixels.get_frame_mut();
                for i in 0..(width * height * 4) {
                    pixels[i as usize] = current_frame[i as usize];
                }
            }
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            while let Ok(command) = receiver.try_recv() {
                match command {
                    Command::Stop => control_flow.set_exit(),
                    Command::Draw(frame) => {
                        info!("drawing");
                        current_frame = frame;
                    }
                };
            }

            window.request_redraw();
        }
    });
}
