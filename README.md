## Tooling
* Install [Rust](https://www.rust-lang.org/tools/install).
* Install [ODA API Rust SDK: TMF634](https://github.com/oda-components/oda-api-sdk-rust#tmf634)
* Install [ODA API Rust SDK: TMF639](https://github.com/oda-components/oda-api-sdk-rust#tmf639)

## Build
```bash
cargo build --workspace
```

## Document
```bash
cargo doc --workspace
```

## Run

### TMF634
```bash
RUST_LOG=info cargo run --package oda_ri_tmf634 --bin tmf634_server &
cargo run --package oda_ri_tmf634 --bin tmf634_client -- --help
RUST_LOG=info cargo run --package oda_ri_tmf634 --bin tmf634_client -- --host localhost ListResourceSpecification
```

### TMF639
```bash
RUST_LOG=info cargo run --package oda_ri_tmf639 --bin tmf639_server &
cargo run --package oda_ri_tmf639 --bin tmf639_client -- --help
RUST_LOG=info cargo run --package oda_ri_tmf639 --bin tmf639_client -- --host localhost ListResource
```

