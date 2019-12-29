use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Colors {
    pending: Vec<String>,
    failure: String,
    success: String,
}

impl Colors {
    pub(crate) fn new(pending: &[&str], failure: &str, success: &str) -> Self {
        Self {
            pending: pending.iter().map(|&arg| arg.to_string()).collect(),
            failure: failure.to_string(),
            success: success.to_string(),
        }
    }

    pub(crate) fn pending(&self) -> &Vec<String> {
        &self.pending
    }

    pub(crate) fn failure(&self) -> &str {
        &self.failure
    }

    pub(crate) fn success(&self) -> &str {
        &self.success
    }
}
