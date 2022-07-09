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
