use std::sync::{Arc, Mutex};
use crate::socket_address::*;
use crate::weight::*;

/// Max number of servers
pub const MAX_SERVERS: usize = 256;

/// Error messages
pub const TOO_MANY_SERVERS: &'static str = "Il numero di server supera i 256";
pub const ZERO_OR_NEGATIVE_SERVERS: &'static str = "Il numero di server è 0 o un numero negativo";
pub const INDEX_PROBLEM: &'static str = "There is a problem with the index";

/// The main struct for storing the Socket Address of each server
/// and perform Round Robin Algorithm
#[derive(Debug)]
pub struct WeightedRoundRobinLB {
    /// stores the Scoket Address of each server
    addresses: Vec<Weight>,
    /// index of the vector, used for concurrent access at the vector
    index: Arc<Mutex<usize>>
}

impl WeightedRoundRobinLB {
    /// Create and return a new instance of WeightedRoundRobinLB struct
    /// # Arguments
    ///
    /// * `servers_number` - the capacity of the inner vector
    pub fn new(servers_number: usize) -> Result<Self, &'static str> {
        if servers_number > MAX_SERVERS {
            return Err(TOO_MANY_SERVERS);
        }
        if servers_number <= 0 {
            return Err(ZERO_OR_NEGATIVE_SERVERS);
        }
        Ok(WeightedRoundRobinLB { 
            addresses: Vec::with_capacity(servers_number),
            index: Arc::new(Mutex::new(0))
        })
    }

    /// Insert a new SocketAddress in the inner vector.
    /// Return an error if the len of vector is already 
    /// at the max capacity MAX_SERVERS.
    /// # Arguments
    ///
    /// * `servers_number` - the capacity of the inner vector
    pub fn insert_socket_address(&mut self, socket_address: SocketAddress, weight: usize) -> Result<(), &'static str> {
        if self.addresses.len() + 1 > MAX_SERVERS {
            return Err(TOO_MANY_SERVERS);
        }
        self.addresses.push(Weight::new(socket_address, weight));
        Ok(())
    }

    /// Implement a multithreaded weighted round robin algorithm.
    /// Return the socket address of the next server.
    /// This operation is thread safe.
    pub fn next_server(&self) -> &SocketAddress {
        let mut idx = self.index.lock().unwrap();
        let option = self.addresses[*idx].next_request();
        
        match option {
            Some(socket_address) => return socket_address,
            None => {
                *idx = (*idx + 1) % self.addresses.len();
                return self.addresses[*idx].next_request().expect(INDEX_PROBLEM);
            }
        }
    }

    /// Return the len of the inner vector
    pub fn n_of_servers(&self) -> usize {
        self.addresses.len()
    }

    // Return the capacity of the inner vector
    pub fn capacity(&self) -> usize {
        self.addresses.capacity()
    }

}