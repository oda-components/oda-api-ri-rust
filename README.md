# Tooling
* Install [Rust](https://www.rust-lang.org/tools/install)
* Install [Redis Stack](https://redis.io/docs/install/install-stack/)
* Install [ODA API Rust SDK: TMF634](https://github.com/oda-components/oda-api-sdk-rust#tmf634)
* Install [ODA API Rust SDK: TMF639](https://github.com/oda-components/oda-api-sdk-rust#tmf639)

# Build

```bash
$ cargo build --workspace
```

# Document

```bash
$ cargo doc --workspace
```

# Run

## Redis

Start the Redis Stack server:
```bash
$ redis-stack-server
```
Initialize an index:
```bash
$ redis-cli
127.0.0.1:6379> FT.CREATE resourceSpecification ON JSON PREFIX 1 resourceSpecification: SCHEMA $.id AS id TAG
```

## TMF634

### Run

```bash
$ RUST_LOG=info cargo run --package oda_ri_tmf634 --bin tmf634_server
$ cargo run --package oda_ri_tmf634 --bin tmf634_client -- --help
$ RUST_LOG=info cargo run --package oda_ri_tmf634 --bin tmf634_client -- --host localhost ListResourceSpecification
```

### Test

> [!WARNING]
> Before run tests required run [Redis](README.md#redis) and create index.

```bash
$ cargo test --package oda_ri_tmf634 --bin tmf634_server -- --nocapture
```

## TMF639

```bash
$ RUST_LOG=info cargo run --package oda_ri_tmf639 --bin tmf639_server
$ cargo run --package oda_ri_tmf639 --bin tmf639_client -- --help
$ RUST_LOG=info cargo run --package oda_ri_tmf639 --bin tmf639_client -- --host localhost ListResource
```
