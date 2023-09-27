## Homework 6 (Smart Home implementation)
### Rust Professional course
To fork __Homework 6__ project, typo:
```commandline
git clone https://github.com/Galaxy13/smart_home/homework6
```
Unit tests are availlible for *smarthome_lib* library. To run tests execute:
```commandline
cargo test --lib
```
or
```commandline
cargo test
```
Examples of usage of *smarthome_lib* are availible in /src/examples directory. To execute examples:
```commandline
cargo run --example smarthome_demo
```

New thing implemented:
- added options to create and remove rooms and devices (with tests)
- added new tests