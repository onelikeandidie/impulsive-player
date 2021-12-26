use std::{thread};

use chrono::{Duration, Utc, Timelike};
use rand::{prelude::SliceRandom, Rng};
use rodio::{Sink, OutputStream};
use util::{config::Config, loader::{load_file_paths, load_audio_file}};
mod util;

#[tokio::main]
pub async fn main() {
    // Deserialize the config
    let config: Config;
    match tokio::fs::read_to_string("./config.toml").await {
        Ok(config_file) => {
            config = toml::from_str(&config_file).unwrap();
        },
        Err(_) => panic!("No config provided or format is not compliant.")
    };
    // Scan the music directory
    let paths = load_file_paths(&config.music_directory);
    // Load files
    // This doesn't work, so I just load everytime a song plays
    // Should be fine? I assume the source is dropped once the loop scope ends
    //let audio_sources = load_audio_files(paths.clone());
    // Play audio files
    let player_handle = tokio::spawn(async move {
        let rng = &mut rand::thread_rng();
        // Create output device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        loop {
            // Minecraft plays music every 5 to 20 minutes after the last one finished
            let delay = Duration::minutes(rng.gen_range(5..20));
            let now = Utc::now();
            let next_song_plays_at = now + delay;
            let (is_pm, hour) = next_song_plays_at.hour12();
            println!("Next song plays on: {:02}:{:02}:{:02} {}",
                hour,
                next_song_plays_at.minute(),
                next_song_plays_at.second(),
                if is_pm { "PM" } else { "AM" }
            );
            thread::sleep(delay.to_std().unwrap());
            // Load a random song
            if let Some(file_path) = paths.choose(rng) {
                println!("Playing: {}", file_path);
                let source = load_audio_file(file_path.to_string());
                sink.append(source);
            }
            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            println!("Waiting for \"music\" to finish");
            sink.sleep_until_end();
        }
    });
    player_handle.await.unwrap();
}