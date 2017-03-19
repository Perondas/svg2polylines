#![crate_type = "dylib"]

extern crate libc;
extern crate svg2polylines;

use std::ffi::CStr;

use libc::{c_char, size_t};
use svg2polylines::{CoordinatePair, parse};

/// Structure that contains a pointer to the coordinate pairs as well as the
/// number of coordinate pairs. It is only used for C interop.
#[repr(C)]
pub struct Polyline {
    ptr: *const CoordinatePair,
    len: size_t,
}

#[no_mangle]
pub extern fn svg_str_to_polylines(
    svg: *const c_char,
    out_vec: *mut *mut Polyline,
    out_vec_len: *mut size_t,
) -> u8 {

    // Convert C string to Rust string
    let c_str = unsafe {
        assert!(!svg.is_null());
        CStr::from_ptr(svg)
    };
    let r_str = c_str.to_str().unwrap();

    // Process
    match parse(r_str) {
        Ok(vec) => {
            println!("Done!");

            // Convert `Vec<Vec<CoordinatePair>>` to `Vec<Polyline>`
            let mut tmp_vec: Vec<Polyline> = vec.iter().map(|v| Polyline {
                ptr: v.as_ptr(),
                len: v.len(),
            }).collect();
            tmp_vec.shrink_to_fit();
            assert!(tmp_vec.len() == tmp_vec.capacity());

            // Return number of polylines
            unsafe { *out_vec_len = tmp_vec.len(); }

            // Move data to heap and return pointer to it
            let boxed = Box::new(tmp_vec);
            unsafe { *out_vec = Box::into_raw(boxed) as *mut Polyline; }

            0
        },
        Err(_) => 1
    }
}
