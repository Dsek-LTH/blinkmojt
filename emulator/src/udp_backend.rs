use std::net::{UdpSocket, ToSocketAddrs};

use crate::{Frame, Blinkmojt};

pub struct UdpFrame {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

impl UdpFrame {
    fn new(width: u32, height: u32) -> UdpFrame {
        UdpFrame {
            width, height, buffer: vec![0; (width * height) as usize]
        }
    }
}

pub struct UdpBlinkmojt {
    width: u32,
    height: u32,
    socket: UdpSocket,
}

impl Blinkmojt for UdpBlinkmojt {
    type Frame = UdpFrame;

    fn get_frame(&mut self) -> Self::Frame {
        UdpFrame::new(self.width, self.height)
    }

    fn draw_frame(&mut self, frame: Self::Frame) {
        self.socket.send(&frame.buffer).unwrap();
    }

    fn close(self) {
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn depth(&self) -> u32 {
        8
    }
}

impl Frame for UdpFrame {
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

        self.buffer[y * (self.width as usize) + x] = (r & 0xff) as u8;
    }
}

pub fn open(addr: impl ToSocketAddrs, _width: u32, _height: u32) -> UdpBlinkmojt {
    let width = 95;
    let height = 7;

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect(addr).unwrap();
    UdpBlinkmojt {
        width, height, socket
    }
}


