use std::{fs::{self, File}, io::BufReader};

use regex::Regex;
use rodio::Decoder;

pub fn load_file_paths(music_dir: &str) -> Vec<String> {
    let paths = fs::read_dir(music_dir).unwrap();
    let mut file_paths: Vec<String> = vec![];
    // Playable formats
    let regex = Regex::new(r"((\.ogg)|(\.mp3)|(\.flac)|(\.wav))$").unwrap();
    for path in paths {
        // Unwrap the filepath into a string!
        let path = path.unwrap().path();
        // Check if path is dir
        if path.is_dir() {
            // Recurse???
            let mut vec_to_add = load_file_paths(path.to_str().unwrap());
            file_paths.append(&mut vec_to_add);
        } else {
            let file_path = String::from(path.to_str().unwrap());
            // Check if the it is a file we can play
            if regex.is_match(&file_path) {
                println!("Added {} to the list", file_path);
                file_paths.push(file_path);
            }
        }
    }
    if file_paths.len() == 0 {
        panic!("No files provided in the music directory!");
    }
    file_paths
}

// Doesn't actually work for some reason!
pub fn load_audio_files(file_paths: Vec<String>) -> Vec<Decoder<BufReader<File>>> {
    let mut audio_sources = vec![];
    for file_path in file_paths {
        audio_sources.push(load_audio_file(file_path));
    }
    audio_sources
}

pub fn load_audio_file(file_path: String) -> Decoder<BufReader<File>> {
    // Load a sound from a file
    let file = BufReader::new(File::open(file_path).unwrap());
    // Decode that sound file into a source
    Decoder::new(file).unwrap()
}