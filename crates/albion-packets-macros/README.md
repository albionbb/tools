# albion-packets-macros

Proc-macros for declarative Albion Online packet decoding in [`albion-packets`](../albion-packets).

## `#[derive(PhotonPacket)]`

Derive a `decode` constructor that maps Photon protocol parameters into a Rust struct.

### Example

```rust
use albion_packets_macros::PhotonPacket;

#[derive(PhotonPacket)]
struct OperationJoinResponse {
    #[photon(index = 0)]
    address: String,
    #[photon(index = 1)]
    port: u16,
}
```

### Field attributes

Every field **must** have a `#[photon(...)]` attribute.

| Attribute | Description |
|-----------|-------------|
| `index = N` | **Required.** Photon parameter index to read from the parameter map. |
| `dict_key = N` | Read a value from a `Dictionary` parameter at the given `index` using this dictionary key. |
| `default = expr` | Expression to use when the parameter (or dictionary key) is missing. If omitted, the field's `Default::default()` is used. |
| `decode_with = "path"` | Decode the raw bytes at `index` with a custom function (e.g. `"decode_custom"`). |
