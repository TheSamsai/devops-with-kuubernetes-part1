use uuid::Uuid;

use chrono::prelude::*;

use std::{thread::sleep, time::Duration};

fn main() {
    let string = Uuid::new_v4().hyphenated().to_string();

    loop {
        let time = Utc::now();

        println!("{}: {}", time, string);

        sleep(Duration::from_secs(5));
    }
}
