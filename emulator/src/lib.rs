pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub mod pixels_backend;

pub trait Frame {
    fn set_pixel(&mut self, position: (i32, i32), color: u32);
    fn draw(self: Box<Self>);
}

pub trait Blinkmojt {
    fn get_frame(&mut self) -> Box<dyn Frame>;
    fn close(self: Box<Self>);
}
