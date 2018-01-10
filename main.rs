extern crate chrono;

mod brute_force;
use brute_force::BruteForce;

fn main() {
    println!("Defaulting to arguments: password: 'zzzzzz', len: 6, brute_force");
    BruteForce::new().init("zzzzzz".to_string());
}
