use std::sync::{Arc, Mutex};
use crate::{
    server::socket_address::*,
    balancers::{
        standard_weighted_load_balancer::weight::*, 
        LoadBalancer
    }
};

// Max number of servers
pub const MAX_SERVERS: usize = 256;

// Error messages
pub static TOO_MANY_SERVERS: &str = "The number of servers is over 256";
pub static ZERO_OR_NEGATIVE_SERVERS: &str = "The number of servers is 0 or a negative number";
pub static INDEX_PROBLEM: &str = "There is a problem with the index";

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

    /// Return the len of the inner vector
    pub fn n_of_servers(&self) -> usize {
        self.addresses.len()
    }

    // Return the capacity of the inner vector
    pub fn capacity(&self) -> usize {
        self.addresses.capacity()
    }

}

impl LoadBalancer for WeightedRoundRobinLB {

    fn new(servers_number: usize) -> Result<Box<Self>, &'static str> {
        if servers_number > MAX_SERVERS {
            return Err(TOO_MANY_SERVERS);
        }
        if servers_number <= 0 {
            return Err(ZERO_OR_NEGATIVE_SERVERS);
        }
        Ok(Box::new(WeightedRoundRobinLB { 
            addresses: Vec::with_capacity(servers_number),
            index: Arc::new(Mutex::new(0))
        }))
    }

    fn insert_socket_address(&mut self, socket_address: SocketAddress, weight: usize) -> Result<(), &'static str> {
        if self.addresses.len() + 1 > MAX_SERVERS {
            return Err(TOO_MANY_SERVERS);
        }
        self.addresses.push(Weight::new(socket_address, weight));
        Ok(())
    }

    fn next_server(&self) -> &SocketAddress {
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
}