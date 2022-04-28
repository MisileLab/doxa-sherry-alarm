use rodio::{Decoder, OutputStream, Source};

use chrono::offset::Local;
use chrono::Timelike;

use serde::Deserialize;

use log::warn;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Config {
    dayhour: u8,
    daymin: u8,
    daysec: u8,
    nighthour: u8,
    nightmin: u8,
    nightsec: u8
}

fn main() {
    let config = read_user_from_file("config.json").unwrap();
    let days = vec![config.dayhour, config.daymin, config.daysec];
    let nights = vec![config.nighthour, config.nightmin, config.nightsec];
    loop {
        let (_, day_stream) = OutputStream::try_default().unwrap();
        let dayfile = BufReader::new(File::open("sounds/goodday.mp3").unwrap());
        let daysource = Decoder::new(dayfile).unwrap();
        let (_, night_stream) = OutputStream::try_default().unwrap();
        let nightfile = BufReader::new(File::open("sounds/goodnight.mp3").unwrap());
        let nightsource = Decoder::new(nightfile).unwrap();
        let nowtime = Local::now();
        let (hour, min, sec) = (nowtime.hour(), nowtime.minute(), nowtime.second());
        if hour as u8 == days[0] && min as u8 == days[1] && sec as u8 == days[2] {
            match day_stream.play_raw(daysource.convert_samples()) {
                Ok(_) => {},
                Err(_) => {
                    warn!("Can't play day audio sample.");
                }
            };
        } else if hour as u8 == nights[0] && min as u8 == nights[1] && sec as u8 == nights[2] {
            match night_stream.play_raw(nightsource.convert_samples()) {
                Ok(_) => {},
                Err(_) => {
                    warn!("Can't play night audio sample.");
                }
            };
        }
    }
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c = serde_json::from_reader(reader)?;

    Ok(c)
}