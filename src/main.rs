use notify_rust::Notification;

use rodio::{Decoder, OutputStream, source::Source};

use chrono::offset::Local;
use chrono::Timelike;

use serde::Deserialize;

use clap::Parser;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;


#[derive(Deserialize, Debug, Parser)]
#[clap(author, about, long_about = None)]
struct Config {
    dayhour: u32,
    daymin: u32,
    daypm: bool,
    nighthour: u32,
    nightmin: u32,
    nightpm: bool
}

fn main() {
    println!("Run alarm!");
    let mut config = match Config::try_parse() {
        Ok(obj) => obj,
        Err(_) => read_user_from_file("config.json").unwrap()
    };
    if config.daypm {
        config.dayhour += 12;
    }
    if config.nightpm {
        config.nighthour += 12;
    }
    let mut days = vec![config.dayhour, config.daymin];
    let mut nights = vec![config.nighthour, config.nightmin];
    if cfg!(debug_assertions) {
        println!("{:#?}", config);
    }
    loop {
        let nowtime = Local::now();
        let (hour, min, sec) = (nowtime.hour(), nowtime.minute(), nowtime.second());
        if cfg!(debug_assertions) {
            println!("{}h {}m {}s", hour, min, sec);
        }
        if hour == days[0] && min == days[1] && sec == 0 {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open("sounds/goodday.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            match stream_handle.play_raw(source.convert_samples()) {
                Ok(_) => {
                    match Notification::new().summary("일어나!")
                        .action("5min", "5분만 더..")
                        .action("on", "일어났어")
                        .show() {
                            Ok(notify) => {
                                #[cfg(all(unix, not(target_os = "macos")))]
                                notify.wait_for_action(|action| match action {
                                    "5min" => {
                                        (days[0], days[1]) = add_minute(days[0], days[1], 5);
                                    },
                                    "on" => {
                                        (days[0], days[1]) = (config.dayhour, config.daymin);
                                    },
                                    "__closed" => {
                                        (days[0], days[1]) = add_minute(days[0], days[1], 5);
                                    },
                                    _ => ()
                                }); 
                            },
                            Err(err) => {
                                println!("Error: {}", err)
                            }
                        }
                    sleep(Duration::from_secs(18));  
                },
                Err(e) => { println!("Error: {:?}", e) }
            };
        } else if hour == nights[0] && min == nights[1] && sec == 0 {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open("sounds/goodnight.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            match stream_handle.play_raw(source.convert_samples()) {
                Ok(_) => {
                    match Notification::new()
                        .icon("assets/goodnight.mp3")
                        .summary("잘 자요.")
                        .action("5min", "5분만 더..")
                        .action("on", "잘게..")
                        .show() {
                            Ok(notify) => {
                                #[cfg(all(unix, not(target_os = "macos")))]
                                notify.wait_for_action(|action| match action {
                                    "5min" => {
                                        (nights[0], nights[1]) = add_minute(nights[0], nights[1], 5);
                                    },
                                    "on" => {
                                        (nights[0], nights[1]) = (config.nighthour, config.nightmin);
                                    },
                                    "__closed" => {
                                        (nights[0], nights[1]) = add_minute(nights[0], nights[1], 5);
                                    },
                                    _ => ()
                                }); 
                            },
                            Err(err) => {
                                println!("Error: {}", err)
                            }
                        }
                    sleep(Duration::from_secs(8));  
                },
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

#[cfg(all(unix, not(target_os = "macos")))]
fn add_minute(mut hour: u32, mut min: u32, amount: u32) -> (u32, u32) {
    min = min + amount;
    if min >= 60 {
        min = min - amount;
        hour = hour + 1;
        if hour >= 12 {
            hour = hour - 12;
        }
    }
    (min, hour)
}