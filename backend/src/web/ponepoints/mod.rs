mod points;
mod pone;

pub use points::*;
pub use pone::*;

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
