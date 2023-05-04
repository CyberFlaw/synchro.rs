impl AudioInput {
    pub fn fetch_audio() -> Result<audio_data, ()>{
        let audio_frame= unsafe { audio_fetch() };

        match audio_frame.buffer_size {
            -1 => Err(()),
            0 => Err(()),
            _ => Ok(audio_frame)
        }
    }
}