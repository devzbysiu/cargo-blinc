use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use std::thread;
use std::time::Duration;

pub struct Transition {
    blinkers: Blinkers,
    transition: Vec<Message>,
    success_msg: Option<Message>,
    failure_msg: Option<Message>,
}

impl From<&str> for Transition {
    fn from(colors: &str) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        let mut transition = Vec::new();
        for color_name in colors.split(' ') {
            transition.push(Message::Fade(
                Color::from(color_name),
                Duration::from_millis(500),
            ));
        }
        Transition {
            blinkers,
            transition,
            success_msg: None,
            failure_msg: None,
        }
    }
}

impl Transition {
    pub fn go(self) {
        let messages = self.transition.clone();
        thread::spawn(move || loop {
            for &message in &messages {
                self.blinkers.send(message).unwrap();
                std::thread::sleep(Duration::from_millis(500));
            }
        });
    }

    pub fn on_success<I: Into<String>>(mut self, color_name: I) -> Self {
        self.success_msg = Some(self.color_msg(color_name));
        self
    }

    fn color_msg<I: Into<String>>(&self, color_name: I) -> Message {
        Message::Fade(
            Color::from(color_name.into().as_str()),
            Duration::from_millis(500),
        )
    }

    pub fn on_failure<I: Into<String>>(mut self, color_name: I) -> Self {
        self.failure_msg = Some(self.color_msg(color_name));
        self
    }
}
