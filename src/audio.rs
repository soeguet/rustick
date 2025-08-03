use kira::sound::PlaybackState;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};
use std::cell::RefCell;
use std::fmt;

const INIT_AUDIO_PATH: &str = "public/new-notification-014-363678.mp3";
const INIT_BREAK_AUDIO_PATH: &str = "public/snd_fragment_retrievewav-14728.mp3";

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
    pub main_audio: RefCell<CustomAudio>,
    pub break_audio: RefCell<CustomAudio>,
}

impl Default for AudioMaster {
    fn default() -> Self {
        let init_audio_path = INIT_AUDIO_PATH.to_owned();
        let sound_data = StaticSoundData::from_file(&init_audio_path).unwrap();
        let main_audio = RefCell::new(CustomAudio {
            audio_data: sound_data,
            audio_path_int: String::new(),
            audio_path: init_audio_path,
        });

        let init_break_audio_path = INIT_BREAK_AUDIO_PATH.to_owned();
        let sound_break_data = StaticSoundData::from_file(&init_break_audio_path).unwrap();
        let break_audio = RefCell::new(CustomAudio {
            audio_data: sound_break_data,
            audio_path_int: String::new(),
            audio_path: init_break_audio_path,
        });
        Self {
            main_audio,
            break_audio,
        }
    }
}

impl AudioMaster {
    pub fn run_main_audio(&self) {
        // Create an audio manager. This plays sounds and manages resources.
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let main_audio_ref = self.main_audio.borrow();
        let cloned_data = main_audio_ref.audio_data.clone();
        let result = manager.play(cloned_data).expect("Could not load audio");

        loop {
            if result.state() != PlaybackState::Playing {
                break;
            }
        }
    }
    pub fn run_break_audio(&self) {
        // Create an audio manager. This plays sounds and manages resources.
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let break_audio_ref = self.break_audio.borrow();
        let cloned_data = break_audio_ref.audio_data.clone();
        let result = manager.play(cloned_data).expect("Could not load audio");

        loop {
            if result.state() != PlaybackState::Playing {
                break;
            }
        }
    }

    pub fn pre_settings_phase(&mut self) {
        let x = self.main_audio.get_mut();
        x.audio_path_int = x.audio_path.clone();
    }

    pub fn set_main_audio_path(&mut self) -> Result<(), AudioError> {
        let x = self.main_audio.get_mut();
        let result = StaticSoundData::from_file(&x.audio_path);

        match result {
            Ok(n) => {
                x.audio_data = n;
                x.audio_path = x.audio_path_int.clone();
                Ok(())
            }
            Err(_) => Err(AudioError {
                message: format!("could not load audio file: \"{}\"", x.audio_path_int),
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
        audio_buffer.run_main_audio();
    }
}
