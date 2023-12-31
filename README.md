<p align="center">
  <h1 align="center">escl-rs</h1>
</p>

**A Rust library for discovering and using scanners via the [eSCL protocol](https://mopria.org/spec-download) (scanning over network)**

[![crates-badge](https://img.shields.io/crates/v/escl.svg)](https://crates.io/crates/escl)

## Features

- [x] LAN service discovery
- [x] Scanner capabilities
- [x] Scanner status
- [x] Scan via the _Pull Scan_ model

## Example

The [example](./examples/scan.rs) uses multicast DNS to look for scanners in LAN for 5 seconds, and scans using the first discovered scanner. There's no need to pre-configure scanner IP address.

The output image is saved as `scan.jpg` in the current directory. To run the example:

```console
cargo run --example scan
```

> [!NOTE]
>
> `escl-rs` supports any scanner URLs, including HTTPS. The use of LAN here is just for simplicity of the example, and to demonstrate LAN discovery capabilities.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
