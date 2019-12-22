use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

const NOT_IMPORTANT: usize = 0;

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
    pub fn go(self) -> Result<Sender<Msg>, failure::Error> {
        let (sender, receiver) = unbounded();
        thread::spawn(move || -> Result<usize, failure::Error> {
            loop {
                match receiver.try_recv() {
                    Ok(Msg::Success) => self.send_success_msg()?,
                    Ok(Msg::Failure) => self.send_failure_msg()?,
                    Err(_) => NOT_IMPORTANT,
                };
                self.play_transition();
            }
        });
        Ok(sender)
    }

    fn send_success_msg(&self) -> Result<usize, failure::Error> {
        println!("blinking with success");
        self.blinkers
            .send(self.success_msg.unwrap_or(Message::Fade(
                Color::from("green"),
                Duration::from_millis(500),
            )))?;
        Ok(NOT_IMPORTANT)
    }

    fn send_failure_msg(&self) -> Result<usize, failure::Error> {
        println!("blinking with failure");
        self.blinkers
            .send(self.failure_msg.unwrap_or(Message::Fade(
                Color::from("red"),
                Duration::from_millis(500),
            )))?;
        Ok(NOT_IMPORTANT)
    }

    fn play_transition(&self) {
        for &message in &self.transition {
            self.blinkers.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
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
