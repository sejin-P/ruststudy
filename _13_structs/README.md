# 궁금증

```rust
enum MyEnum {
None,
Some(u8),
}
```
여기서 niche optimization을 통해 None에 0을 배치하여 size가 u8을 안넘기도록 한다고 했는데, 만약 u8로 가능한 모든 값을 넣는다면?
