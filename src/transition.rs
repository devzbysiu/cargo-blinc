use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
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
    pub fn go(self) -> Sender<Msg> {
        let messages = self.transition.clone();
        let (sender, receiver) = unbounded();
        thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(Msg::Success) => {
                    self.blinkers
                        .send(self.success_msg.unwrap_or(Message::Fade(
                            Color::from("green"),
                            Duration::from_millis(500),
                        )))
                        .unwrap();
                    break;
                }
                Ok(Msg::Failure) => {
                    self.blinkers
                        .send(self.failure_msg.unwrap_or(Message::Fade(
                            Color::from("red"),
                            Duration::from_millis(500),
                        )))
                        .unwrap();
                    break;
                }
                _ => {}
            }
            for &message in &messages {
                self.blinkers.send(message).unwrap();
                std::thread::sleep(Duration::from_millis(500));
            }
        });
        sender
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

pub enum Msg {
    Success,
    Failure,
}
