use std::env::{var};

pub fn get_home_dir() -> String {
    match var("HOME") {
        Err(why) => {
            println!("Assuming windows due to HOME: {}", why);
            match var("USERPROFILE") {
                Err(why) => panic!("No USERPROFILE found: {}", why),
                Ok(home) => return home,
            }
        },
        Ok(home) => return home,
    };
}