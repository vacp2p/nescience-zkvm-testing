use crate::account::Key;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum InputVisibiility {
    // A public account
    Public,
    // A private account
    Private(Option<Key>),
}
