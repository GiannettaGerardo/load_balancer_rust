mod load_balancer;
mod tests;

use std::{sync::Arc, thread};
use load_balancer::*;

fn main() {
    let mut balancer = WeightedRoundRobinLB::new(5).unwrap();
    balancer.insert_socket_address(SocketAddress(String::from("127.0.0.1"), String::from("9000"))).unwrap();
    balancer.insert_socket_address(SocketAddress(String::from("192.168.1.34"), String::from("8080"))).unwrap();
    balancer.insert_socket_address(SocketAddress(String::from("192.168.0.0"), String::from("80"))).unwrap();
    balancer.insert_socket_address(SocketAddress(String::from("21.78.123.45"), String::from("7890"))).unwrap();
    balancer.insert_socket_address(SocketAddress(String::from("189.24.255.255"), String::from("26748"))).unwrap();
    
    let balancer = Arc::new(balancer);
    
    let mut handles = vec![];

    // test with threads
    for j in 0..5 {
        let balancer = Arc::clone(&balancer);
        let handle = thread::spawn(move || {
            for i in 0..6 {
                println!("T: {}, {}: {}", j,  i, balancer.next_server());
            }
            println!("");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
