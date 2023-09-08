# Interfaces

An **interface** describes a single-focus, composable contract, through which components can interact with each other and with hosts. Interfaces describe the types and functions used to carry out that interaction. For example:

* A "receive HTTP requests" interface might have only a single "handle request" function, but contain types representing incoming requests, outgoing responses, HTTP methods and headers, and so on.
* A "wall clock" interface might have two functions, one to get the current time and one to get the granularity of the timer. It would also include a type to represent an instant in time.
