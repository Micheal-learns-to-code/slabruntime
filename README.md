# slabruntime

`slabruntime` is a Rust library designed to provide runtime support for the CP210x USB to UART bridge controllers. This library offers a set of functionalities to interact with and manage CP210x devices efficiently.
The library is based on the API specification in this document [CP210x USB to UART API Specification](https://www.silabs.com/documents/public/application-notes/an978-cp210x-usb-to-uart-api-specification.pdf).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
slabruntime = "0.1.0"
```

## Usage

Import the `slabruntime`. The starting point for all the functionality is to create a CP210x object. With a CP210x object, you can get the number of cp210x devices connected to the system, open the devices, retrieve more information from them.

```rust
use slabruntime::CP210x;

fn main() {
    // Initialize the CP210x library
    let cp210x_runtime = Cp210xRuntime::new().expect("Something wrong with the library: ");

    // Perform operations with the devices
    // ...
}
```

## Documentation

For detailed documentation, please visit [docs.rs/slabruntime](https://docs.rs/slabruntime).

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss your ideas.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
