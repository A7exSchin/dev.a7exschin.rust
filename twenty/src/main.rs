use std::time::{Duration};
use notify_rust::Notification;
use std::{thread, io};
use std::io::prelude::*;

use iced::widget::{container, slider, column, text};
use iced::{Element, Center, Fill};

fn main() -> iced::Result {
    iced::application("Twenty", Twenty::update, Twenty::view)
    .window_size(iced::Size::new(300.0, 300.0))
    .run()
}

#[derive(Default)]
struct Twenty {
    timeout: u8,
    timer: u8,
}

#[derive(Debug, Clone)]
enum Message {
    TimerSliderChanged(u8),
    TimeoutSliderChanged(u8)
}

impl Twenty {

    fn new() -> Self {
        Twenty {
            timeout: 20,
            timer: 20,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TimerSliderChanged(value) => {
                self.timer = value;
            }
            Message::TimeoutSliderChanged(value) => {
                self.timeout = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let timeout_slider = container(
            slider(5..= 60, self.timeout, Message::TimeoutSliderChanged)
                .default(self.timeout)
                .shift_step(1),
        )
        .width(200);

        let timer_slider = container(
            slider(1..= 60, self.timer, Message::TimerSliderChanged)
                .default(self.timer)
                .shift_step(1),
        )
        .width(200);

        let timeout_text = text(format!("Timeout: {} seconds", self.timeout));
        let timer_text = text(format!("Timer: {} minutes", self.timer));

        column![timeout_text, timeout_slider, timer_text, timer_slider]
            .spacing(20)
            .padding(20)
            .width(Fill)
            .align_x(Center)
            .into()
    }
}

fn twenty() {
    println!("Starting Twenty Eye Care Timer");

    let timeout_input = std::env::args().nth(1).expect("Missing timeout argument").parse::<u64>().expect("Invalid timeout value");
    let timer_input = std::env::args().nth(2).expect("Missing timer argument").parse::<u64>().expect("Invalid timer value");
    
    let timeout = Duration::new(timeout_input, 0);
    let timer = Duration::new(timer_input*60, 0);

    let msg = format!("Take a {} second break in {} minutes. Press Enter to stop the timer.", timeout.as_secs(), timer.as_secs()/60);
    println!("{}", msg);

    let handler = thread::spawn(move || {
        loop {
            let notif_msg = format!("Take a break! Look away from the screen for {} seconds.", timeout.as_secs());
            thread::sleep(timer);
            println!("Time to take a break! Look away from the screen.");
            Notification::new()
                .summary("Take a Break!")
                .body(&notif_msg)
                .timeout(timeout.as_secs() as i32)
                .show()
                .unwrap();
            thread::sleep(timeout);
        }
    });
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to end... \n").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
