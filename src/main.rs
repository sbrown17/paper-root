use chrono::prelude::*;
use rand::prelude::*;
use wallpaper;

fn main() {
    println!("Starting paper-root...");
    let date_time_string = Utc::now().to_string();
    println!("{}", date_time_string);
    // get season and time of day (eg. winter, midday)
    // grab vec of image filenames within that path
    // random number from 1..veclength
    // set wallpaper
    // wallpaper::set_from_path
}
