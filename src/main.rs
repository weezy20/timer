#![allow(unused, warnings)]

use indicatif::ProgressBar;
use std::{process, thread};

use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Config {
    /// Hours , default value 0
    #[structopt(short, long = "hour", default_value = "0")]
    hours: u32,

    /// Minutes, default value 0
    #[structopt(short, long = "min", default_value = "0")]
    min: u32,

    /// Seconds, default value 0
    #[structopt(short, long = "secs", default_value = "0")]
    seconds: u32,
}
fn main() {
    let cfg = Config::from_args();
    let Config {
        hours: hour,
        min: min,
        seconds: secs,
    } = cfg;
    // println!("hours : {}\nminutes : {}\nseconds: {}", hour, min, secs);
    let time_in_seconds = hour * 3600 + min * 60 + secs;
    make_timer(time_in_seconds);
}
fn make_timer(time: u32) {
    let pb = ProgressBar::new(time.into());
    pb.println("Timer started..");
    for i in 1..=time {
        thread::sleep(Duration::from_secs(1));
        pb.inc(1);
    }

    pb.finish();
    println!("Time's up!");
}
