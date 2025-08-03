use kira::sound::PlaybackState;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};
use std::fmt;

#[derive(Debug)]
pub struct AudioError {
    pub message: String,
}
impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AudioError: {}", self.message)
    }
}

impl std::error::Error for AudioError {}

pub struct CustomAudio {
    pub audio_path: String,
    pub audio_path_int: String,
    audio_data: StaticSoundData,
}

pub struct AudioMaster {
    pub(crate) main_audio: CustomAudio,
    break_audio: CustomAudio,
}

impl Default for AudioMaster {
    fn default() -> Self {
        let init_audio_path = "public/happy-message-ping-351298.mp3".to_owned();
        let sound_data = StaticSoundData::from_file(&init_audio_path).unwrap();
        let main_audio = CustomAudio {
            audio_data: sound_data,
            audio_path_int: String::new(),
            audio_path: init_audio_path,
        };

        let init_audio_path = "public/happy-message-ping-351298.mp3".to_owned();
        let sound_data = StaticSoundData::from_file(&init_audio_path).unwrap();
        let break_audio = CustomAudio {
            audio_data: sound_data,
            audio_path_int: String::new(),
            audio_path: init_audio_path,
        };

        Self {
            main_audio,
            break_audio,
        }
    }
}

impl AudioMaster {
    pub fn run_audio(&self) {
        // Create an audio manager. This plays sounds and manages resources.
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let result = manager
            .play(self.main_audio.audio_data.clone())
            .expect("Could not load audio");

        loop {
            if result.state() != PlaybackState::Playing {
                break;
            }
        }
    }

    pub fn pre_settings_phase(&mut self) {
        self.main_audio.audio_path_int = self.main_audio.audio_path.clone();
    }

    pub fn set_main_audio_path(&mut self, new_audio_path: String) -> Result<(), AudioError> {
        let result = StaticSoundData::from_file(&self.main_audio.audio_path);

        match result {
            Ok(n) => {
                self.main_audio.audio_data = n;
                self.main_audio.audio_path = new_audio_path;
                Ok(())
            }
            Err(err) => Err(AudioError {
                message: format!("could not load audio file: \"{}\"", new_audio_path),
            }),
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
