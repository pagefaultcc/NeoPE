# NeoPE

## What is this?
A cross-platform lightweight PE utility library written in Rust.

## Usage
Run the buffer example:
```bash
cargo run --example from_buffer
```

Or add `neope` to your `Cargo.toml`:
```rust
use neope::{PE, PE_FAILED};

let pe = PE::new(&bytes);
if PE_FAILED(pe.GetError()) {
    // Handle error
}

if let Some(dos_header) = pe.GetDosHeader() {
    println!("e_lfanew: {}", dos_header.e_lfanew);
}
```

## Contributing
Contributions are welcome! Whether it's fixing bugs, improving documentation, adding new features, or optimizing existing code, your help is appreciated.

## License
This project is distributed under the MIT License. For more information, see [LICENSE](https://github.com/pagefaultcc/NeoPE/blob/main/LICENSE).
