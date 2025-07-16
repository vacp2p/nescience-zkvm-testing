use serde::{Deserialize, Serialize};
use crate::types::{AuthenticationPath, Key};

#[derive(Serialize, Deserialize)]
pub enum InputVisibiility {
    // A public account
    Public,
    // A private account
    Private(Option<(Key, AuthenticationPath)>),
}
