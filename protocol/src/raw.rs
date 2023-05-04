use serde::{Serialize, Deserialize};
// use rodio;

// note: all these must be typecased from the os::raw types into rust types
#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
struct Protocol{
    // the actual size of the audio buffer that will be sent.
    // 0 <= size <= 255 : may be tweeked to u16 if found nessary
    buffer_size: u8,

    // this should only be set to `true` if
    // 1. a stop user inturrupt is passed and
    // 2. it is the last frame conatining the remaining stream
    // AudioInput.terminate can be used to define this
    fin: bool,

    // this is the source of the audio coming from the server
    // it can be alsa device, fifo path or pulse source
    // this maynot be needed but adding this in case
    source: &str,

    // this is input mode, it sepecifes which audio server the data comes from
    // it can be alsa?, fifo, pulse, portaudio, shmem, sndio
    input_method: i8,

    //the data itself, stream of double floating point integers
    stream: [f64; 255],

    // used to check the order in which the packets arrive
    // similar to sliding window, use it with mod(size(u32)) to prevent overflows
    packet_flag: u32
}