# Bion

Unified facade for the Bion crate ecosystem.

## Crates

| Crate | Description |
|-------|-------------|
| `bion` | Root facade re-exporting all sub-crates |
| `bion-genesis` | Genesis and bootstrapping |
| `bion-cortex` | Cortical processing |
| `bion-connectome` | Connectome structures |
| `bion-membrane` | Membrane boundary layer |
| `bion-soma` | Soma core |

## Usage

```rust
use bion::cortex::add;
```

Depend on the full stack:

```toml
[dependencies]
bion = "0.1"
```

Or use an individual crate:

```toml
[dependencies]
bion-cortex = "0.1"
```

## Publishing to crates.io

1. Log in: `cargo login`
2. Publish sub-crates first (order among them does not matter):

   ```sh
   cargo publish -p bion-genesis
   cargo publish -p bion-cortex
   cargo publish -p bion-connectome
   cargo publish -p bion-membrane
   cargo publish -p bion-soma
   ```

3. Publish the facade last:

   ```sh
   cargo publish -p bion
   ```

Dry-run before each publish: `cargo publish -p <crate> --dry-run`

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
