use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct AudioMaster {
    break_file: String,
    // break_over_file: Decoder<BufReader<File>>,
}

impl Default for AudioMaster {
    fn default() -> Self {
        Self {
            break_file: "public/happy-message-ping-351298.mp3".into(),
        }
    }
}

impl AudioMaster {
    pub fn play_audio(&self) {
        // Get an output stream handle to the default physical sound device.
        // Note that the playback stops when the stream_handle is dropped.
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(&self.break_file).unwrap());
        // Note that the playback stops when the sink is dropped
        let result = rodio::play(&stream_handle.mixer(), file);

        match result {
            Ok(n) => {
                n.sleep_until_end();
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::audio::AudioMaster;

    #[test]
    fn test_progress_calculation_half() {
        let master = AudioMaster::default();
        master.play_audio();
    }
}
