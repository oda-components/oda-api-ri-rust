## Tooling
* Install [Rust](https://www.rust-lang.org/tools/install).
* Install [ODA API Rust SDK: TMF639](https://github.com/oda-components/oda-api-sdk-rust#tmf639)

# Build
Building the ODA Open API RI for Rust.
```bash
cargo buid
```

## Terminal 1: Server
```bash
cargo run --bin server
cargo run --bin server -- --https
```

## Terminal 2: Client
```bash
cargo run --bin client -- --help
cargo run --bin client UnregisterListener
cargo run --bin client DeleteResource
cargo run --bin client ListResource
cargo run --bin client RetrieveResource
```

## Document
In each API subdirectory:
```bash
cargo doc
```
