use serde_derive::Deserialize;
use serde_derive::Serialize;
use transition::Led;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Colors {
    pending: Vec<Led>,
    failure: Led,
    success: Led,
}

impl Colors {
    pub(crate) fn new(pending: Vec<Led>, failure: Led, success: Led) -> Self {
        Self {
            pending,
            failure,
            success,
        }
    }

    pub(crate) fn pending(&self) -> &[Led] {
        &self.pending
    }

    pub(crate) fn failure(&self) -> &Led {
        &self.failure
    }

    pub(crate) fn success(&self) -> &Led {
        &self.success
    }
}
