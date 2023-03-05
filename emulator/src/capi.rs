
use std::{os::raw::c_char, ffi::c_int, str::FromStr};

use crate::{Blinkmojt, Frame};

type BackendBlinkmojt = crate::udp_backend::UdpBlinkmojt;
type BackendFrame = crate::udp_backend::UdpFrame;

#[repr(C)]
pub struct blinkmojt_t(BackendBlinkmojt);

#[repr(C)]
pub struct frame_t(BackendFrame);

#[repr(C)]
pub struct blinkmojt_info_t {
    pub width: c_int,
    pub height: c_int,
    pub depth: c_int,
}

fn parse_or<E, V: FromStr>(result: Result<String, E>, default: V) -> V {
    match result {
        Err(_) => default,
        Ok(s) => s.parse().unwrap_or(default)
    }
}

#[no_mangle]
pub extern "C" fn blinkmojt_open(_name: *const c_char) -> *const blinkmojt_t {
    let width = parse_or(std::env::var("BLINK_WIDTH"), 64);
    let height = parse_or(std::env::var("BLINK_HEIGHT"), 32);
    let depth = parse_or(std::env::var("BLINK_DEPTH"), 32);
    let addr = parse_or(std::env::var("BLINK_ADDR"), "127.0.0.1:1337".to_string());

    let mojt = Box::new(blinkmojt_t(
        //crate::pixels_backend::open(width, height)
        crate::udp_backend::open(addr, width, height, depth)
    ));

    Box::into_raw(mojt)
}

#[no_mangle]
pub extern "C" fn blinkmojt_get_info(mojt: *const blinkmojt_t, info: *mut blinkmojt_info_t) {
    unsafe {
        (*info).width = (*mojt).0.width() as c_int;
        (*info).height = (*mojt).0.height() as c_int;
        (*info).depth = (*mojt).0.depth() as c_int;
    }
}

#[no_mangle]
pub extern "C" fn blinkmojt_close(mojt: *mut blinkmojt_t) {
    unsafe {
        let mojt = Box::from_raw(mojt);
        mojt.0.close();
    }
}

#[no_mangle]
pub extern "C" fn blinkmojt_get_frame(mojt: *mut blinkmojt_t) -> *const frame_t {
    let mojt = Box::new(frame_t(
        unsafe { (*mojt).0.get_frame() }
    ));

    Box::into_raw(mojt)
}

#[no_mangle]
pub extern "C" fn blinkmojt_draw_frame(mojt: *mut blinkmojt_t, frame: *mut frame_t) {
    unsafe {
        let frame = Box::from_raw(frame);
        (*mojt).0.draw_frame(frame.0);
    }
}

#[no_mangle]
pub extern "C" fn frame_set_pixel(frame: *mut frame_t, x: c_int, y: c_int, color: u32) {
    unsafe {
        (*frame).0.set_pixel((x, y), color);
    }
}
