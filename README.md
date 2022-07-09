# load_balancer_rust

A simple Load Balancer for HTTP request, implemented with a simple Weighted Round Robin algorithm

# To use it

Set the file config.json with 2 informations:

- "Listen_to": object that represents the server socket address and must have 2 fields:
   - "ipv4": string representing IPv4 address
   - "port": string representing port number

- "Servers": an array of objects that represents the servers that will be used for the balancing. This object must have 3 informations:
   - "ipv4": string representing IPv4 address
   - "port": string representing a port number
   - "weight": number representing a weight for this server in the balancer

For shutting down the server press CTRL+C

# Algorithms implemented for load balancing

##- Simple Weighted Round Robin

There is an array of socket addresses and each socket address has a number (weight).
Iter through the array mod array.len() (it's like having a circular array). Each socket address in the iteration is repeated "weight" times.
- Disadvantages
    - Use of 2 mutexes, one for the array index and one for the weight index.
