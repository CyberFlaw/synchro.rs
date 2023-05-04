use std::os::raw::{c_char, c_int, c_uint, c_double};

// uncomment the link for linking the extern function
// #[link(name = "audioinput", kind = "static")]
extern "C" {
    pub fn audio_fetch() -> AudioInput;
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioInput {
    pub stream: *mut c_double,
    pub buffer_size: c_int,

    pub format: c_int,
    pub rate: c_uint,
    pub channels: c_uint,
    pub source: *mut c_char,
    pub im: c_int,                               // input method
    pub error_messaage: [c_char; 1024],
    pub sample_counter: c_int,
    pub terminate: c_int
}

#[derive(Debug)]
pub struct AudioOutput {}
