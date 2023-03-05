
use std::{os::raw::c_char, ffi::c_int};

use crate::{Blinkmojt, Frame};

#[repr(C)]
pub struct blinkmojt_t(crate::pixels_backend::PixelsBlinkmojt);

#[repr(C)]
pub struct frame_t(crate::pixels_backend::PixelsFrame);

#[repr(C)]
pub struct blinkmojt_info_t {
    pub width: c_int,
    pub height: c_int,
    pub depth: c_int,
}

#[no_mangle]
pub extern "C" fn blinkmojt_open(name: *const c_char) -> *const blinkmojt_t {
    let mojt = Box::new(blinkmojt_t(
        crate::pixels_backend::open(64, 32)
    ));

    Box::into_raw(mojt)
}

#[no_mangle]
pub extern "C" fn blinkmojt_get_info(mojt: *const blinkmojt_t, info: *mut blinkmojt_info_t) {
    unsafe {
        (*info).width = (*mojt).0.width as c_int;
        (*info).height = (*mojt).0.height as c_int;
        (*info).depth = 32;
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
