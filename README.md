## Tooling
* Install [Rust](https://www.rust-lang.org/tools/install).
* Install [ODA API Rust SDK: TMF634](https://github.com/oda-components/oda-api-sdk-rust#tmf634)
* Install [ODA API Rust SDK: TMF639](https://github.com/oda-components/oda-api-sdk-rust#tmf639)

# Build
Building the ODA Open API RI for Rust.
```bash
cargo buid
```
# TMF634
## Terminal 1: Server
```bash
cargo run -p oda_634_ri --bin server
cargo run -p oda_634_ri --bin server -- --https
```

## Terminal 2: Client
```bash
cargo run -p oda_634_ri --bin client -- --help
cargo run -p oda_634_ri --bin client UnregisterListener
cargo run -p oda_634_ri --bin client DeleteExportJob
cargo run -p oda_634_ri --bin client ListExportJob
cargo run -p oda_634_ri --bin client RetrieveExportJob
cargo run -p oda_634_ri --bin client DeleteImportJob
cargo run -p oda_634_ri --bin client ListImportJob
cargo run -p oda_634_ri --bin client RetrieveImportJob
cargo run -p oda_634_ri --bin client DeleteResourceCandidate
cargo run -p oda_634_ri --bin client ListResourceCandidate
cargo run -p oda_634_ri --bin client RetrieveResourceCandidate
cargo run -p oda_634_ri --bin client DeleteResourceCatalog
cargo run -p oda_634_ri --bin client ListResourceCatalog
cargo run -p oda_634_ri --bin client RetrieveResourceCatalog
cargo run -p oda_634_ri --bin client DeleteResourceCategory
cargo run -p oda_634_ri --bin client ListResourceCategory
cargo run -p oda_634_ri --bin client RetrieveResourceCategory
cargo run -p oda_634_ri --bin client DeleteResourceSpecification
cargo run -p oda_634_ri --bin client ListResourceSpecification
cargo run -p oda_634_ri --bin client RetrieveResourceSpecification
```

# TMF639
## Terminal 1: Server
```bash
cargo run -p oda_639_ri --bin server
cargo run -p oda_639_ri --bin server -- --https
```

## Terminal 2: Client
```bash
cargo run -p oda_639_ri --bin client -- --help
cargo run -p oda_639_ri --bin client UnregisterListener
cargo run -p oda_639_ri --bin client DeleteResource
cargo run -p oda_639_ri --bin client ListResource
cargo run -p oda_639_ri --bin client RetrieveResource
```

## Document
In each API subdirectory:
```bash
cargo doc
```
