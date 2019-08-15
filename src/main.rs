extern crate ratelimit;

use std::thread;
use std::time::Duration;

fn main() {
    let mut ratelimit = ratelimit::Builder::new()
        .capacity(1) // number of tokens the bucket will hold
        .quantum(1) // add five tokens per interval
        .interval(Duration::new(5, 0)) // add `quantum` tokens every 30 seconds
        .build();
    thread::spawn(move || ratelimit.run());

    thread::sleep(Duration::new(500, 0));
}
