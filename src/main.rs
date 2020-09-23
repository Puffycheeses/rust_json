use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod lib;

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<()> {
    let home = lib::home::get_home_dir();
    let data_path = Path::new(&home).join(".data.json");
    let display = data_path.display();

    if data_path.exists() {
        let mut file = match File::open(&data_path) {
            Err(why) => panic!("Could not open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut existing_data = String::new();
        match file.read_to_string(&mut existing_data) {
            Err(why) => panic!("Could not read file: {}", why),
            Ok(existing_data) => existing_data,
        };
        let existing_data_json: Person = serde_json::from_str(&existing_data)?;
        println!("Hello, {}", existing_data_json.name);
        println!("Found JSON: {}", existing_data)
    } else {
        println!("Could not find {}", display)
    }

    Ok(())
}
