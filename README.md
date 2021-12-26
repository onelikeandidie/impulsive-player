# Impulsive Player

This app plays music .mp3, .flac, .ogg and .wav at seemingly random time
intervals simulating minecraft's music player.

You must provide the music yourself. These music files must be in the path in
your config file. The program will scan recursively on startup, meaning that
you can store your songs inside other directories in that music directory.

# State

The program only runs in command line since I haven't made a GUI for it.
This is a hobby project, I may never update it again but I will try to listen
to issues created here.

# Roadmap

- [X] Recursevely select songs in the given directory
- [ ] Decode music files at startup
- [ ] Time interval in config
- [ ] GUI for volume control
- [ ] Hotkey support
- [ ] %Chance config support for each song
- [ ] Hot reload song list or watch for directory changes
- [ ] Integration with popular streaming services (not priority)

# FAQ

## AAAAA IMPULSIVE PLAYER CRASHED!!! WHAT DO I DO?!?!?

There's a couple of reasons this could be:

1. No config file was provided, a file with the name `config.toml` needs to be
    created before startup for the program to work.
2. The config file provided did not contain the field:
    `music_directory = "./music"`.
3. No music files were found in the directory given. Make sure you put your 
    "high-quality" music files in the directory you set in the config file.

## I've started the impulsive player, when will my music play?

On launch, the program delays even the first song between 5 to 20 minutes until
it starts playing. If you really want to know, the program also prints out the 
next timestamp of when the next song is going to play.

## What's the time interval between songs?

The music will be played in random intervals from 5 to 20 minutes after the 
last song was played!

## Can I put my music files in different directories?

No, but you can create directories inside the music directory given in the
config file! Impulsive Player will search recusively and create a list of all
the files in there regardless of directory structure.

# Build

This program runs on rust, the programming language. To build it you must have
rust installed and run `cargo build` to create a binary or `cargo run` for a 
temporary debug binary.

The program will crash if a config file was not provided.

# Run

Download the latest release for you current OS and run it from your terminal.

> _This section needs expanding._

# Final Notes

If you want to make a similar app, feel free to take inspiration from the code
written here. The crates I used were:

- [`Rodio`](https://crates.io/crates/rodio) for the loading and playback of 
    music.
- [`Chrono`](https://crates.io/crates/chrono) for calculating time until the
    the next song is played.
- [`Tokio`](https://crates.io/crates/tokio) for creating a thread that contains
    music player.
- [`Serde`](https://crates.io/crates/serde) for loading config files.
- [`Rand`](https://crates.io/crates/rand) for choosing random intervals and 
    random files to play
- [`Regex`](https://crates.io/crates/regex) to filter only playable files from
    the music directory.

Have fun with this app! I felt like having music play as I'm programming 
seemingly at random made some work feel somewhat less meaningless :).