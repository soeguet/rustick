use kira::sound::PlaybackState;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};

pub struct AudioMaster {
    audio_data: StaticSoundData,
}

impl Default for AudioMaster {
    fn default() -> Self {
        let sound_data =
            StaticSoundData::from_file("public/happy-message-ping-351298.mp3").unwrap();

        Self {
            audio_data: sound_data,
        }
    }
}

impl AudioMaster {
    pub fn run_audio(&self) {
        // Create an audio manager. This plays sounds and manages resources.
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let result = manager
            .play(self.audio_data.clone())
            .expect("Could not load audio");

        loop {
            if result.state() != PlaybackState::Playing {
                break;
            }
        }
    }
}

// add test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer_default() {
        let audio_buffer = AudioMaster::default();
        audio_buffer.run_audio();
    }
}
