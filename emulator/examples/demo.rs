use blinkemu::pixels_backend::open;

pub fn main() {
    let mut mojt = open(32, 16);
    let mut frame = mojt.get_frame();
    frame.set_pixel((2, 3), 0xff00ff00);
    frame.draw();
    std::thread::sleep(std::time::Duration::from_secs(4));
    mojt.close();
}