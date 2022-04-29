use rodio::{Decoder, OutputStream, source::Source};

use chrono::offset::Local;
use chrono::Timelike;

use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Config {
    dayhour: u8,
    daymin: u8,
    nighthour: u8,
    nightmin: u8
}

fn main() {
    println!("Run alarm!");
    let config = read_user_from_file("config.json").unwrap();
    let days = vec![config.dayhour, config.daymin];
    let nights = vec![config.nighthour, config.nightmin];
    loop {
        let nowtime = Local::now();
        let (hour, min, sec) = (nowtime.hour(), nowtime.minute(), nowtime.second());
        if cfg!(debug_assertions) {
            println!("{}h {}m {}s", hour, min, sec);
        }
        if hour as u8 == days[0] && min as u8 == days[1] && sec as u8 == 0 {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open("sounds/goodday.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            match stream_handle.play_raw(source.convert_samples()) {
                Ok(_) => { 
                    sleep(Duration::from_secs(16)) 
                },
                Err(e) => { println!("Error: {:?}", e) }
            };
        } else if hour as u8 == nights[0] && min as u8 == nights[1] && sec as u8 == 0 {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open("sounds/goodnight.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            match stream_handle.play_raw(source.convert_samples()) {
                Ok(_) => { sleep(Duration::from_secs(8)) },
                Err(e) => { println!("Error: {:?}", e) }
            };
        }
        sleep(Duration::from_secs(1));
    }
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c = serde_json::from_reader(reader)?;

    Ok(c)
}