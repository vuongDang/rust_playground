# Microservices

Architecture where services of a system are split into different entities.
The services are usually isolated into a VM or a container and communicate
with each other through a network communication protocol.
Opposed to monolythic services where all the codebase is ran by a single entity.

Pros: scalability, reliability, easier during development due to service separation
Cons: harder to setup, slower due to communication, more complex

## Docker containers

Most popular tool for containers. A competitor is Buildah.
Docker uses Linux namespaces and cgroups (control groups) to work.
Docker on MacOS uses a VM while Windows is either a VM or WSL.
Docker runs natively on Linux.
Containers shares the host OS hence consumes less resources than VMs.

### Namespaces

Namespaces provide a layer of isolation at kernel level by separating
different aspects of the system:

- mount/mnt, virtually partitions the file system. Similar to `chroot`
  but at the kernel level making it more secure
- process/pid, cuts of a branch from pid 1 and will make it a virtual root process
  in the namespace. This prevents accessing the processes above in the tree
- ipc, controls whether process can talk to each other
- net, controls which network devices a process can see
- user, allow to have a virtual root in the namespace without root access to the main system
- UTS, controls hostname and domain information making processes to think
  they're running on differently named servers

### cgroups

Linux kernel features that monitors and controls resource usage for a collection of processes.
CPU, memory, disk, I/O, PID, devices...
Latest version is cgroups v2 released in 2023.

### Docker tools/terminology

- image: immutable template to run a container
- container: running instance of an image
- Docker Engine: server tasked with managing containers and providing a runtime
- Dockerfile: recipe to buid a Docker image
- Docker CLI: CLI to manage containers
- Docker Desktop: GUI to manage containers
- Docker Hub: registry to share and pull images
- Docker Compose: orchestration tool to manage and coordinate multiple containers on a single host
- Docker Swarm: orchestration tool to manage and scale containers. Creates a cluster of Docker engines
  called a swarm. A swarm is composed of nodes which are physical or virtual machines.

Note: Docker Swarm and Kubernetes fulfill the same role. Kubernetes is better suited
for large scale applications with finer-grained control while Docker Swarm is simpler
and is well-integrated with the Docker suite of tools.

## Communication protocol

- gRPC
- REST
- Web Sockets
- Message queues
- Publisher-subscriber

### gRPC

- made by Google
- relies on HTTP/2 under the hood
- uses ProtoBuf as serialized format
- ProtoBuf data is defined in a schema by the server side
- Client side uses ProtoBuf code "stubs" with the schema to query data
- not used much on the web because HTTP/2 while enabled the browsers
  do not let devs the control to implement gRPC
  - there is gRPC-web that converts HTTP/2 to HTTP/1 but does not support
    streaming or multiplexing (features of HTTP/2)
- supports load balancing, tracing, health checking and auth
- well suited for:
  - microservices architecture
  - low-latency inter service communication : real-time data or event-driven architecture

#### Pros

- cross-platform
- multiplexing
- bidirectional streaming
- lightweight messages compared to text-based formats (JSON, or XML)
- high-performance encoding
- strongly typed message structure

#### Cons

- limited browser support
- protobuf is hard to debug due to being not text-based
- header weight
- relatively new

### Web Sockets

- W3C standard
- started by an HTTP/1 request that is upgraded by the server to a
  full-duplex, bidirectional channel
- can transmit both text and binary data
- built for streaming on the web

### Message passing

- Designed for async communication and distributed environment
- scalable, usually performs better under heavy loads than traditionnal RPC systems
- reliable, ensures fault tolerance
- Generally slower than RPC protocols due to introduction of a middleware between nodes
- suited for:
  - continuous streams of data such as real-time data processing of events from diverse sensors
  - distributed systems
- RabbitMQ, Apache Kafka, Amazon Sqs

#### Publisher-Subscribers

- Message passing pattern where publishers broadcast message to all their subscribers
- To be used when you want all your consuming applications to receive a copy of the message

#### Message queues

- Message passing pattern where publishers send messages to one/mutliple queues
- Message consumers may share these queues and retrieve the messages inside
- To be used to distribute work and you just need a single consumer to process each messages
