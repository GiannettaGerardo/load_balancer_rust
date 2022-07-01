use std::sync::{Arc, Mutex};

use crate::socket_address::*;

#[derive(Debug)]
pub struct Weight {
    socket_address: SocketAddress,
    weight: usize,
    request_counter: Mutex<usize>
}

impl Weight {
    pub fn new(socket_address: SocketAddress, weight: usize) -> Self {
        Weight {
            socket_address,
            weight,
            request_counter: Mutex::new(0)
        }
    }

    pub fn next_request(&self) -> Option<&SocketAddress> {
        let mut counter = self.request_counter.lock().unwrap();
        if *counter == self.weight {
            *counter = 0;
            return None;
        }
        *counter += 1;
        Some(&self.socket_address)
    }
}