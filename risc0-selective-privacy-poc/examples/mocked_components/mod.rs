use core::types::Key;

use crate::mocked_components::client::MockedClient;

pub mod client;
pub mod sequencer;

/// Default users for examples
pub const USER_CLIENTS: [MockedClient; 3] = [
    MockedClient::new([1; 8]),
    MockedClient::new([2; 8]),
    MockedClient::new([3; 8]),
];
