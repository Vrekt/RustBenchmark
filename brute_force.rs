extern crate chrono;

use std;
use chrono::Utc;

pub struct BruteForce;

impl BruteForce {
    pub fn new() -> Self {
        BruteForce {}
    }

    pub fn init(&self, password: String) {
        println!("Password set to: {}", password);
        println!("Starting local thread...");

        let mut next = password.clone();
        let mut now = Utc::now().timestamp();
        let mut attempts = 0;

        let thread = std::thread::Builder::new().spawn(move || loop {
            next = get_next(next);
            attempts += 1;

            if next == password {
                break;
            }
            let current = Utc::now().timestamp();
            let time = current - now;
            if time >= 5 {
                now = current;
                println!("Currently running benchmark... attempts: {} current: {}", attempts, next);
            }

            continue;
        }).expect("Couldn't spawn thread.");

        thread.join().expect("Couldn't join!");

        let after = Utc::now().timestamp();
        let elapsed = now - after;

        println!("Thread job finished. {}", elapsed);
    }
}

pub fn get_next(mut next: String) -> String {
    let len = next.len();
    let ch = next.pop().expect("Expected a character.");

    if ch == 'z' {
        if len > 1 {
            get_next(next) + "a"
        } else {
            "aa".to_string()
        }
    } else {
        next.push((ch as u8 + 1) as char);
        next
    }
}
