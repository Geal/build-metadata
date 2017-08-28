# Build metadata

use this crate to embed repository and build metadata at compile time

This project uses procedural macros to provide you with this metadata,
exposing three macros:

* `head!()`: the name of the current git branch or tag you are in
* `commit!()`: short git commit id
* `time!()`: UTC build time. This value is cached, so it will stay the same for every call in one crate. Note that using this makes the build non reproducible (since the resulting binary will change depending on build time)

## Usage

Import it with cargo from `crates.io` by adding this to you Cargo.toml` file:

```toml
[dependencies]
build_metadata = "^0.1"
```

```rust
#![feature(plugin)]
#![plugin(build_metadata)]

fn main() {
    println!("build time: {}", time!());
    println!("head: {}", head!());
    println!("commit id: {}", commit!());
}
```

