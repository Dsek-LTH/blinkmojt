pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub mod pixels_backend;

pub trait Frame {
    fn set_pixel(&mut self, position: (i32, i32), color: u32);
}

pub trait Blinkmojt {
    type Frame: Frame;

    fn get_frame(&mut self) -> Self::Frame;
    fn draw_frame(&mut self, frame: Self::Frame);
    fn close(self);
}
