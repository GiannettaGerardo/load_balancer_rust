use std::{sync::{Arc, Mutex}, fmt::{Display, self}};

pub const MAX_SERVERS: usize = 256;
pub const TOO_MANY_SERVERS: &'static str = "Il numero di server supera i 256";
pub const ZERO_OR_NEGATIVE_SERVERS: &'static str = "Il numero di server è 0 o un numero negativo";

#[derive(Debug)]
pub struct SocketAddress(pub String, pub String);

impl SocketAddress {
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(20);
        s.push_str(&self.0);
        s.push(':');
        s.push_str(&self.1);
        return s;
    }
}

impl Display for SocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct WeightedRoundRobinLB {
    addresses: Vec<SocketAddress>,
    index: Arc<Mutex<usize>>
}

impl WeightedRoundRobinLB {
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

    pub fn insert_socket_address(&mut self, socket_address: SocketAddress) -> Result<(), &'static str> {
        if self.addresses.len() + 1 > MAX_SERVERS {
            return Err(TOO_MANY_SERVERS);
        }
        self.addresses.push(socket_address);
        Ok(())
    }

    pub fn next_server(&self) -> &SocketAddress {
        let mut idx = self.index.lock().unwrap();
        *idx = (*idx + 1) % self.addresses.len();
        &self.addresses[*idx]
    }

    pub fn n_of_servers(&self) -> usize {
        self.addresses.len()
    }

    pub fn capacity(&self) -> usize {
        self.addresses.capacity()
    }

}