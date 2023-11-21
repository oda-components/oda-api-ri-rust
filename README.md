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
cargo run -p oda_ri_tmf634 --bin server
cargo run -p oda_ri_tmf634 --bin server -- --https
```

## Terminal 2: Client
```bash
cargo run -p oda_ri_tmf634 --bin client -- --help
cargo run -p oda_ri_tmf634 --bin client UnregisterListener
cargo run -p oda_ri_tmf634 --bin client DeleteExportJob
cargo run -p oda_ri_tmf634 --bin client ListExportJob
cargo run -p oda_ri_tmf634 --bin client RetrieveExportJob
cargo run -p oda_ri_tmf634 --bin client DeleteImportJob
cargo run -p oda_ri_tmf634 --bin client ListImportJob
cargo run -p oda_ri_tmf634 --bin client RetrieveImportJob
cargo run -p oda_ri_tmf634 --bin client DeleteResourceCandidate
cargo run -p oda_ri_tmf634 --bin client ListResourceCandidate
cargo run -p oda_ri_tmf634 --bin client RetrieveResourceCandidate
cargo run -p oda_ri_tmf634 --bin client DeleteResourceCatalog
cargo run -p oda_ri_tmf634 --bin client ListResourceCatalog
cargo run -p oda_ri_tmf634 --bin client RetrieveResourceCatalog
cargo run -p oda_ri_tmf634 --bin client DeleteResourceCategory
cargo run -p oda_ri_tmf634 --bin client ListResourceCategory
cargo run -p oda_ri_tmf634 --bin client RetrieveResourceCategory
cargo run -p oda_ri_tmf634 --bin client DeleteResourceSpecification
cargo run -p oda_ri_tmf634 --bin client ListResourceSpecification
cargo run -p oda_ri_tmf634 --bin client RetrieveResourceSpecification
```

# TMF639
## Terminal 1: Server
```bash
cargo run -p oda_ri_tmf639 --bin server
cargo run -p oda_ri_tmf639 --bin server -- --https
```

## Terminal 2: Client
```bash
cargo run -p oda_ri_tmf639 --bin client -- --help
cargo run -p oda_ri_tmf639 --bin client UnregisterListener
cargo run -p oda_ri_tmf639 --bin client DeleteResource
cargo run -p oda_ri_tmf639 --bin client ListResource
cargo run -p oda_ri_tmf639 --bin client RetrieveResource
```

