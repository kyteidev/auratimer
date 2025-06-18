use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use tracing::error;

pub fn play_sound(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.to_string();

    thread::spawn(move || {
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let file = BufReader::new(File::open(&path)?);
            let source = Decoder::new(file)?;

            stream_handle.play_raw(source.convert_samples())?;

            thread::sleep(std::time::Duration::from_secs(1));
            Ok(())
        })();

        let _ = result.map_err(|e| {
            error!("Failed to play sound: {}", e);
        });
    });

    Ok(())
}

pub fn play_alarm() {
    let _ = play_sound("assets/alarm.wav").map_err(|e| {
        error!("Failed to play alarm sound: {}", e);
    });
}
