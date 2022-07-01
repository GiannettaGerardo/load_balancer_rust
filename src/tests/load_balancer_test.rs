#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};
    use dashmap::DashMap;
    use crate::load_balancer::*;
    use crate::socket_address::*;

    fn add_soc_addr(wrrlb: &mut WeightedRoundRobinLB) -> Result<(), &'static str> {
        wrrlb.insert_socket_address(
            SocketAddress::new(String::from("127.0.0.1"), String::from("9000"))
                .unwrap(),
                1
        )
    }

    fn create_filled_load_balancer() -> WeightedRoundRobinLB {
        let dim = 5;
        let mut b = WeightedRoundRobinLB::new(dim).unwrap();
        for _ in 0..dim {
            add_soc_addr(&mut b).unwrap();
        }
        return b;
    }

    #[test]
    fn multiple_tests() {
        let fixture = create_filled_load_balancer();
        assert_eq!(fixture.n_of_servers(), 5, "We're testing that address.len == 5");
        assert_eq!(fixture.n_of_servers(), fixture.capacity(), "We're testing that address.len == address.cap");
    }

    #[test]
    fn max_number_of_servers_has_max_capacity() {
        let fixture = WeightedRoundRobinLB::new(MAX_SERVERS);
        match fixture {
            Ok(fixture) => assert_eq!(fixture.capacity(), MAX_SERVERS),
            Err(_) => panic!("max_number_of_servers_has_max_capacity")
        }
    }

    #[test]
    fn zero_servers_return_string_error() {
        let fixture = WeightedRoundRobinLB::new(0);
        match fixture {
            Err(e) => assert_eq!(ZERO_OR_NEGATIVE_SERVERS, e),
            Ok(_) => panic!("n257_servers_return_error")
        }
    }

    #[test]
    fn max_plus_one_servers_return_string_error() {
        let fixture = WeightedRoundRobinLB::new(MAX_SERVERS + 1);
        match fixture {
            Err(e) => assert_eq!(TOO_MANY_SERVERS, e),
            Ok(_) => panic!("n257_servers_return_error")
        }
    }

    #[test]
    fn push_max_plus_one_server_should_return_error() {
        let mut fixture = WeightedRoundRobinLB::new(MAX_SERVERS).unwrap();
        for _ in 0..MAX_SERVERS {
            match add_soc_addr(&mut fixture) {
                Ok(()) => continue,
                Err(_) => panic!("push_max_plus_one_server_should_return_error -> Err arm")
            }
        }
        match add_soc_addr(&mut fixture) {
            Ok(()) => panic!("push_max_plus_one_server_should_return_error -> Ok arm"),
            Err(e) => assert_eq!(e, TOO_MANY_SERVERS)
        }
    }

    #[test]
    fn round_robin_each_of_5_socket_addr_should_handle_6_request() {
        let servers_number = 5; 
        let mut balancer = WeightedRoundRobinLB::new(servers_number).unwrap();

        let one = String::from("127.0.0.1:9000");
        let two = String::from("192.168.1.34:8080");
        let three = String::from("192.168.0.0:80");
        let four = String::from("21.78.123.45:7890");
        let five = String::from("189.24.255.255:26748");
        
        balancer.insert_socket_address(
            SocketAddress::new(String::from("127.0.0.1"), String::from("9000")).unwrap(), 
            1
        ).unwrap();
        balancer.insert_socket_address(
            SocketAddress::new(String::from("192.168.1.34"), String::from("8080")).unwrap(),
            2
        ).unwrap();
        balancer.insert_socket_address(
            SocketAddress::new(String::from("192.168.0.0"), String::from("80")).unwrap(), 
            3
        ).unwrap();
        balancer.insert_socket_address(
            SocketAddress::new(String::from("21.78.123.45"), String::from("7890")).unwrap(),
            1
        ).unwrap();
        balancer.insert_socket_address(
            SocketAddress::new(String::from("189.24.255.255"), String::from("26748")).unwrap(),
            2
        ).unwrap();
        
        let balancer = Arc::new(balancer);
        let hash = Arc::new(DashMap::new());
        let mut handles = vec![];

        for _ in 0..servers_number {

            let balancer = Arc::clone(&balancer);
            let hash = Arc::clone(&hash);

            let handle = thread::spawn(move || {
                for _ in 0..(servers_number+1) {
                    let mut counter = hash.entry(balancer.next_server().get()).or_insert(0);
                    *counter += 1;
                }
            });

            handles.push(handle);
        }
        // join all the thread
        for handle in handles {
            handle.join().unwrap();
        }

        // ASSERTS
        assert_eq!(servers_number, hash.len(), "There isn't 'servers_number' SocketAddress in DashMap");

        assert_eq!(4, *hash.get(&one).unwrap());
        assert_eq!(8, *hash.get(&two).unwrap());
        assert_eq!(9, *hash.get(&three).unwrap());
        assert_eq!(3, *hash.get(&four).unwrap());
        assert_eq!(6, *hash.get(&five).unwrap());

    }
}