## Homework 7 (TCP Power Plug)
### Rust Professional course
To fork __Homework 7__ project, typo:
```commandline
git clone https://github.com/Galaxy13/smart_home/homework7
```
To execute server:
```commandline
cargo run --bin test_server
```
To execute client:
```commandline
cargo run --bin test_client
```
!Client expects an error, if server is not available, but this exception crashes the program

List of available client commands:
```commandline
command[state/get] argument
```
- state on/off : changes power state
- get state/power/name: returns to console current power state/power consumption/plug name
- exit: closes program