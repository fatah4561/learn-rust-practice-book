To create lib crate use the command:

```rust
cargo new <lib_name> --lib
```

then add in the main cargo.toml dependencies:

```toml
lib_name = { path = "lib_name" }
```
