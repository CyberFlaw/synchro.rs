// use serde::{Serialize, Deserialize};

// note: all these must be typecased from the os::raw types into rust types
// TODO: fix serde
// #[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct Protocol<'a>{
    // the actual size of the audio buffer that will be sent.
    // 0 <= size <= 255 : may be tweeked to u16 if found nessary
    pub buffer_size: u8,

    // this should only be set to `true` if
    // 1. a stop user inturrupt is passed and
    // 2. it is the last frame conatining the remaining stream
    // AudioInput.terminate can be used to define this
    pub fin: bool,

    // this is the source of the audio coming from the server
    // it can be alsa device, fifo path or pulse source
    // this maynot be needed but adding this in case
    pub source: &'a str,

    // this is input mode, it sepecifes which audio server the data comes from
    // it can be alsa?, fifo, pulse, portaudio, shmem, sndio
    pub input_method: i8,

    //the data itself, stream of double floating point integers
    pub stream: [f64; 255],

    // used to check the order in which the packets arrive
    // similar to sliding window, use it with mod(size(u32)) to prevent overflows
    pub packet_flag: u32
}

//TODO: implement the logic
impl Protocol<'_> {
    // pub fn new(buffer_size: u8, fin: bool, source: &str, im: i8, stream: [f64; 255], flag: u32) -> Protocol {
    //     Protocol
    // }

    pub fn serialize(&self) {}

    pub fn deserialize(&self) {}
}