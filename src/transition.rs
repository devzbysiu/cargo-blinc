use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use failure::Error;
use std::time::Duration;

pub struct Transition {
    blinkers: Blinkers,
    messages: Vec<Message>,
}

impl From<&str> for Transition {
    fn from(colors: &str) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        let mut messages = Vec::new();
        for color_name in colors.split(' ') {
            messages.push(Message::Fade(
                Color::from(color_name),
                Duration::from_millis(500),
            ));
        }
        Transition { blinkers, messages }
    }
}

impl Transition {
    pub fn go(self) -> Result<(), Error> {
        for message in self.messages {
            self.blinkers.send(message)?;
            std::thread::sleep(Duration::from_millis(500));
        }
        Ok(())
    }
}
