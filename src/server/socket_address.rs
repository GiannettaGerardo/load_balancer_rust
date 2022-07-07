use std::fmt::{Display, self};
use regex::Regex;

pub const IPV4_REGEX: &'static str = r"^(\d{1,3}\.){3}\d{1,3}$";
pub const IPV4_ERROR: &'static str = "Invalid IPv4 address";
pub const PORT_NUMBER_ERROR: &'static str = "Invalid Port Number";


/// Stores a socket address
/// # Arguments
///
/// * `0` - IPv4 address
/// * `1` - Port number
#[derive(Debug)]
pub struct SocketAddress(String, String);

impl SocketAddress {
    /// Create and return a new instance of SocketAddress tuple struct
    /// # Arguments
    ///
    /// * `ipv4` - ipv4 address
    /// * `port_number` - port number
    pub fn new(ipv4: String, port_number: String) -> Result<Self, &'static str> {
        let ipv4_reg = Regex::new(IPV4_REGEX).unwrap();

        // check IPv4
        if !ipv4_reg.is_match(&ipv4) {
            return Err(IPV4_ERROR);
        }
        let bytes_splitted = ipv4.split(".");
        for byte in bytes_splitted {
            let byte_parsed: Result<i32, _> = byte.parse();
            match byte_parsed {
                Err(_) => return Err(IPV4_ERROR),
                Ok(n) => if n > 255 {return Err(IPV4_ERROR);}
            };
        }

        // check port number
        let parsed_port_number: Result<i32, _> = port_number.parse();
        match parsed_port_number {
            Err(_) => return Err(PORT_NUMBER_ERROR),
            Ok(n) => if n >= (1 << 16) {return Err(PORT_NUMBER_ERROR);}
        };

        Ok(SocketAddress(ipv4, port_number))
    }
 
    /// Return port number
    pub fn get_port_number(&self) -> &String {
        &self.1
    }

    /// Return ipv4 address
    pub fn get_ipv4(&self) -> &String {
        &self.0
    }

    /// Return the complete socket address with format IPv4:PORT
    pub fn get(&self) -> String {
        let mut s = String::with_capacity(20);
        s.push_str(&self.0);
        s.push(':');
        s.push_str(&self.1);
        return s;
    }
}

impl Display for SocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}