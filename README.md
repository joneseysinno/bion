# Bion

Typed reactive graph data backend. One dependency, feature-gated layers.

## Crates

| Crate | Description |
|-------|-------------|
| `bion` | Facade with feature-gated re-exports (`soma`, `pns`, `cns`, `full`) |
| `bion-soma` | Shared alphabet — `#![no_std]`, zero I/O, no `bion-*` imports |
| `bion-pns` | Peripheral nervous system — infinite-db gateway and transport adapters |
| `bion-cns` | Central nervous system — circuit execution via injected PNS traits |

```
infinite-db (external)
      ↑
bion-pns  →  bion-cns  →  bion (facade)

bion-soma  ← imported by bion-pns and bion-cns
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for dependency rules and architectural laws. Canonical term namespaces live in [VOCABULARY.md](VOCABULARY.md).

## Usage

Type vocabulary only:

```toml
[dependencies]
bion = { version = "0.1", default-features = false, features = ["soma"] }
```

```rust
use bion::soma::{Impulse, NeuronId, SignalType};
```

Frontend connector (PNS gateway + transport):

```toml
[dependencies]
bion = { version = "0.1", features = ["pns"] }
```

Full backend with execution:

```toml
[dependencies]
bion = { version = "0.1", features = ["full"] }
```

```rust
use bion::cns::{start, shutdown, Circuit};
use bion::pns::{PnsReader, PnsWriter, ImpulseEnvelope};
```

Or depend on an individual crate:

```toml
[dependencies]
bion-soma = "0.1"
bion-pns = "0.1"
bion-cns = "0.1"
```

## Development

```sh
cargo check --workspace --all-features
cargo test --workspace --all-features
```

CI enforces soma purity, CNS/db boundary separation, vocabulary disjointness, and the one-function-per-file rule in `bion-pns` and `bion-cns`.

## Publishing to crates.io

1. Log in: `cargo login`
2. Publish sub-crates first (order among them does not matter):

   ```sh
   cargo publish -p bion-soma
   cargo publish -p bion-pns
   cargo publish -p bion-cns
   ```

3. Publish the facade last:

   ```sh
   cargo publish -p bion
   ```

Dry-run before each publish: `cargo publish -p <crate> --dry-run`

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
