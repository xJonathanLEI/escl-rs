<p align="center">
  <h1 align="center">escl-rs</h1>
</p>

**A Rust library for using scanners via the [eSCL protocol](https://mopria.org/spec-download) (scanning over network)**

[![crates-badge](https://img.shields.io/crates/v/escl.svg)](https://crates.io/crates/escl)

## Features

- [x] Scanner capabilities
- [x] Scanner status
- [x] Scan via the _Pull Scan_ model

## Example

Set your scanner IP address in the `SCANNER_IP` environment variable. For example:

```console
export SCANNER_IP="192.168.1.100"
```

> [!NOTE]
>
> This example assumes that your scanner serves eSCL endpoints at `http://scanner.ip.address/eCSL/xxxx`.
>
> `escl-rs` supports any scanner URLs, including HTTPS. The use of LAN IP here is just for simplicity of the example.

Then run the `scan` example:

```console
cargo run --example scan
```

An output image file `scan.jpg` will be saved in the current directory.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
