use std::os::raw::{c_char, c_float, c_int, c_uint, c_double};

// uncomment the link for linking the extern function
// #[link(name = "audioinput", kind = "static")]
extern "C" {
    fn audio_fetch() -> audio_data;
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioInput {
    stream: *mut c_double,
    buffer_size: c_int,

   format: c_int,
    rate: c_uint,
    channels: c_uint,
    source: *mut c_char,
    im: c_int,                               // input method
    error_messaage: [c_char; 1024],
    sample_counter: c_int,
    terminate: c_int
}

#[derive(Debug)]
pub struct AudioOutput {}
