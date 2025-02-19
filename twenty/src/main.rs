use notify_rust::Notification;
use std::thread::sleep;
use std::time::{Duration};

fn main() {
    println!("Starting Twenty!");

    //let mut minute_timer = Duration::from_minutes(20);
    let minute_timer = Duration::from_secs(5);

    loop {
        sleep(minute_timer);

        Notification::new()
            .summary("Twenty!")
            .body("Twenty minutes have passed! Look outside!")
            .show()
            .unwrap();
    }
}
