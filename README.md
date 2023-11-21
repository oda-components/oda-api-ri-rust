## Tooling
* Install [Rust](https://www.rust-lang.org/tools/install).
* Install [ODA API Rust SDK: TMF634](https://github.com/oda-components/oda-api-sdk-rust#tmf634)
* Install [ODA API Rust SDK: TMF639](https://github.com/oda-components/oda-api-sdk-rust#tmf639)

# Build
```bash
cargo build --workspace
```

## Document
```bash
cargo doc --workspace
```

# TMF634
## Terminal 1: Server
```bash
cargo run -p oda_ri_tmf634 --bin tmf634_server
cargo run -p oda_ri_tmf634 --bin tmf634_server -- --https
```

## Terminal 2: Client
```bash
cargo run -p oda_ri_tmf634 --bin tmf634_client -- --help
cargo run -p oda_ri_tmf634 --bin tmf634_client ListResourceSpecification
```

# TMF639
## Terminal 1: Server
```bash
cargo run -p oda_ri_tmf639 --bin tmf639_server
cargo run -p oda_ri_tmf639 --bin tmf639_server -- --https
```

## Terminal 2: Client
```bash
cargo run -p oda_ri_tmf639 --bin tmf639_client -- --help
cargo run -p oda_ri_tmf639 --bin tmf639_client ListResource
```

