use serde::{Deserialize, Serialize};
use serde_json;
use screenshots::Screen;
use std::io::prelude::*;
use std::fs::File;
use image::{GenericImageView, ImageError};
use rdev::{simulate, EventType};

mod convert;

#[derive(Debug, Serialize, Deserialize)]
struct Coordinates {
    bottom_left: [i32; 2],
    top_right: [i32; 2],
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    coordinates: Coordinates,
    check_delay: u64,
    nexus_key: String,
}

fn main() -> Result<(), ImageError> {

    // Read config.json
    let mut file = File::open("config.json").expect("Impossible d'ouvrir le fichier");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Impossible de lire le fichier");

    // Deserialize the loaded json file
    let config: Config = serde_json::from_str(&contents).unwrap();
    let coordinates = config.coordinates;
    let check_delay = config.check_delay;
    let nexus_key = config.nexus_key;
    let key_press = EventType::KeyPress(convert::string_to_key(&nexus_key));
    let key_release = EventType::KeyRelease(convert::string_to_key(&nexus_key));

    let bottom_left = coordinates.bottom_left;
    let top_right = coordinates.top_right;
    let width: u32 = (top_right[0] - bottom_left[0]).try_into().unwrap();
    let height: u32 = (bottom_left[1] - top_right[1]).try_into().unwrap();
    
    let screen = Screen::from_point(0, 0).unwrap();

    loop {
        //////////////// GET SCREENSHOT ////////////////

        let image = screen.capture_area(bottom_left[0], bottom_left[1], width, height).unwrap();
        image.save("cache/health.png").unwrap();


        //////////////// GET PIXEL COLOR ////////////////

        // Load the image from file
        let img = image::open("cache/health.png")?;

        // Get the color of a specific pixel (0, 0 in this example)
        let pixel_color = img.get_pixel(0, 0);

        // Extract the red value from the pixel color
        let red_value = pixel_color[0];

        // Define all the threshold values
        let red_threshold = 220;
        //let white_threshold = 200;

        // Print the result
        if red_value > red_threshold /*|| (pixel_color[0] > white_threshold && pixel_color[1] > white_threshold && pixel_color[2] > white_threshold)*/ {
            send(&key_press);
            send(&key_release);
            println!("Pressed the nexus key.");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        std::thread::sleep(std::time::Duration::from_millis(check_delay));
    }   

    //Ok(())
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(simulate_error) => {
            println!("We could not send {:?} due to {:?}", event_type, simulate_error);
        }
    }
}