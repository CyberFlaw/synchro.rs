use crate::audio::AudioInput;
use crate::audio::audio_fetch;

impl AudioInput {
    pub fn fetch_audio() -> Result<AudioInput, ()>{
        let audio_frame= unsafe { audio_fetch() };

        match audio_frame.buffer_size {
            -1 => Err(()),
            0 => Err(()),
            _ => Ok(audio_frame)
        }
    }
}