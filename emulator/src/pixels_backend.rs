const PIXEL_SCALE: f64 = 8.0;

use log::{info, error};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    window::Window,
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::WindowBuilder,
};
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
    pub width: u32,
    pub height: u32,
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

    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        let scaled_size = LogicalSize::new(width as f64 * PIXEL_SCALE, height as f64 * PIXEL_SCALE);
        WindowBuilder::new()
            .with_title("Blinkmojt Emulator")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let thread = std::thread::spawn(move || {
        run(width, height, receiver, window);
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

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn depth(&self) -> u32 {
        32
    }
}

fn run(width: u32, height: u32, receiver: Receiver<Command>, window: Window) {

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };
    pixels.set_clear_color(pixels::wgpu::Color::BLACK);

    while let Ok(command) = receiver.try_recv() {
        match command {
            Command::Stop => break,
            Command::Draw(frame) => {
                {
                    let pixels = pixels.get_frame_mut();
                    for i in 0..(width * height * 4) {
                        pixels[i as usize] = frame[i as usize];
                    }
                }
                if let Err(err) = pixels.render() {
                    error!("pixels.render() failed: {}", err);
                    return;
                }
            }
        };
    }
}
