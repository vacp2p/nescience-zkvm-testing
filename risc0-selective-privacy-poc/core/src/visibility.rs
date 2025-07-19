use crate::types::{AuthenticationPath, Key};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AccountVisibility {
    // A public account
    Public,
    // A private account
    Private(Option<(Key, AuthenticationPath)>),
}
