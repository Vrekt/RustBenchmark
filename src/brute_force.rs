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
		let mut before = Utc::now().timestamp();
		let mut attempts = 0;

		let thread = std::thread::Builder::new().spawn(move || loop {
			next = get_next(next);
			attempts += 1;

			if next == password {
				break;
			}
			if attempts % 10_000000 == 0 {
				println!("Currently running benchmark... attempts: {} current: {}", attempts, next);
			}
		}).expect("Couldn't spawn thread.");

		thread.join().expect("Couldn't join!");

		let after = Utc::now().timestamp();
		let elapsed = after - before;

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
