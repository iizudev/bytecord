<p align="center">
  <img alt="bytecord icon" src="bytecord.png" width="240" />
</p>
<p align="center">
  A lightweight utility library for reading and building binary data with alignment.
</p>
<hr />

## Usage

Add this library to your project by:

```bash
cargo add bytecord
```

### Example

reading:

```rust
use bytecord::ByteCord;

let data = vec![0u8; 1024];
let cord = ByteCord::new(data);

// Read with 4-byte alignment
let mut reader = cord.read_with_alignment(4);
let a = cord.next_be_u32().unwrap();
let b = cord.next_n(a).unwrap();
```

building:

```rust
use bytecord::ByteCordBuilder;

// a new builer with alignment of 1 byte (meaning no alignment)
let mut data = ByteCordBuilder::new(1);
data.append_le_u32(1111);
data.append_u8(1);
data.append_le_i64(-3919);
let slice = data.into_boxed_slice();
```

## License

This project is dual-licensed under:

- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE-2.0)
