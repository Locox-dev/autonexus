use serde::{Deserialize, Serialize};
use serde_json;
use screenshots::Screen;
use std::io::prelude::*;
use std::fs::File;
use rdev::{simulate, EventType};

mod convert;

#[derive(Debug, Serialize, Deserialize)]
struct Coordinates {
    bottom_left: [i32; 2],
    top_right: [i32; 2],
}

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    coordinates: Coordinates,
    check_delay: u64,
    nexus_key: String,
    target_colors: Colors,
}

fn main() {

    // Read config.json
    let mut file = File::open("config.json").expect("Could not find config.json file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not open config.json file");

    // Deserialize the loaded json file
    let config: Config = serde_json::from_str(&contents).unwrap();
    let coordinates = config.coordinates;
    let check_delay = config.check_delay;
    let nexus_key = config.nexus_key;
    let target_colors = config.target_colors;
    let key_press = EventType::KeyPress(convert::string_to_key(&nexus_key));
    let key_release = EventType::KeyRelease(convert::string_to_key(&nexus_key));
    let red_target = target_colors.red;
    let green_target = target_colors.green;
    let blue_target = target_colors.blue;

    let bottom_left = coordinates.bottom_left;
    let top_right = coordinates.top_right;
    let width: u32 = (top_right[0] - bottom_left[0]).try_into().unwrap();
    let height: u32 = (bottom_left[1] - top_right[1]).try_into().unwrap();
    
    let screen = Screen::from_point(0, 0).unwrap();

    loop {
        //////////////// GET SCREENSHOT ////////////////

        let image = screen.capture_area(bottom_left[0], bottom_left[1], width, height).unwrap();

        //////////////// GET PIXEL COLOR ////////////////

        let pixel_color = image.get_pixel(0, 0);

        let red_value = pixel_color[0];
        let green_value = pixel_color[1];
        let blue_value = pixel_color[2];

        // Use the line below to check the target values
        //println!("{} ; {} ; {}", red_value, green_value, blue_value);

        // DO WE NEXUS? THAT IS THE QUESTION
        if red_value == red_target && green_value == green_target && blue_value == blue_target {
            send(&key_press);
            send(&key_release);
            println!("Pressed the nexus key.");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        std::thread::sleep(std::time::Duration::from_millis(check_delay));
    }   
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(simulate_error) => {
            println!("We could not send {:?} due to {:?}", event_type, simulate_error);
        }
    }
}