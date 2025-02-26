# Concurrency

Playing with different concurrency models in Rust

## Concurrency models

- OS Threads
- Asynchronous Programing
- Coroutines
- Actor Model
- Event-Driven Programing

## Threads

One of the most basic models which is notably used by OSes.
Threads are spawned within a single progress and can run independently.
Usually threads have their own stacks but share the heap.

### Pros

- easy to implement
- good for CPU-bound tasks

### Cons

- hard to manage
- race conditions and deadlocks happen easily
- threads can be resource intensive

## Event-Driven Programing

From my memories it was made popular with NodeJS.
The idea is to have amain event loop that listen to events
and trigger the corresponding event-handler routines.
This is used a lot for graphical interfaces.

### Pros

- good for I/O bound tasks

### Cons

- Hard to maintain and read
- "Callback hell" for complex code
- Not suitable for CPU-bound tasks

## Actor Model

Multiple independent units of computations called "actors". They synchronize
with each other with message passing.

### Pros

- No shared state, reducing race conditions
- Scales well in distributed systems since the model is close to the reality

### Cons

- Less intuitive
- Message passing can be slower

## Software Transactional Memory (STM)

STM is a concurrency model that allows multiple threads to operate on shared memory without using locks.
It uses the "transaction" abstraction that ensure that memory operations within a transaction are atomic.

### Pros

- simplifies reasoning about concurrent code
- reduces the chances of deadlocks

### Cons

- Performance can be an issue depending on the situation

## Green threads, coroutines, futures, generators

Those are variations of threads but at the programing language level.
They are designed for concurrency with 1 system threads with N virtual threads.
They are more lightweight than system threads since the context switching is handled
by the programmer or the programming language runtime.

One of the important point is that context switching is pushed by the programmer
rather than the system scheduler.
This prevents context switching during critical operations and avoids deadlocks or
race conditions.
This also enables better concurrency since these units of computation willingly switch
between each other.

Implementations and exact definitions of these concepts differ between programing languages.

## Concurrency in Rust

In Rust OS threads and async tasks are the main concurrency models used.

### Comparisons

#### OS threads

- mangaged by the OS
- preemptively scheduled
- higher resource usage
- suited to small amount of CPU-bound work
- harder to reason about

#### Async tasks

- managed by the async crate used
- cooperatively scheduled
- smaller resource used
- suited for large amount of I/O-bound workloads
- easier to reason about
