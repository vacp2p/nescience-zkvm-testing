use serde::{Deserialize, Serialize};

pub type PrivateKey = [u32; 8];

#[derive(Serialize, Deserialize)]
pub enum InputVisibiility {
    // A public account
    Public,
    // A private account
    Private(Option<PrivateKey>),
}

