use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use failure::Error;
use std::time::Duration;

pub struct Transition {
    blinkers: Blinkers,
    transitions: Vec<Blink>,
}

impl From<&str> for Transition {
    fn from(colors: &str) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        let mut transitions = Vec::new();
        for color_name in colors.split(' ') {
            transitions.push(Blink::new(color_name, 500));
        }
        Transition {
            blinkers,
            transitions,
        }
    }
}

impl Transition {
    pub fn go(self) -> Result<(), Error> {
        for transition in self.transitions {
            self.blinkers
                .send(Message::Fade(transition.color, transition.duration))?;
            std::thread::sleep(Duration::from_millis(500));
        }
        Ok(())
    }
}

struct Blink {
    color: Color,
    duration: Duration,
}

impl Blink {
    pub fn new(color: &str, duration: u64) -> Self {
        Blink {
            color: Color::from(color),
            duration: Duration::from_millis(duration),
        }
    }
}
