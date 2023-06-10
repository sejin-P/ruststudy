# 궁금증

```rust
enum MyEnum {
None,
Some(u8),
}
```
여기서 niche optimization을 통해 None에 0을 배치하여 size가 u8을 안넘기도록 한다고 했는데, 만약 u8로 가능한 모든 값을 넣는다면?
->

When you use niche optimizations in Rust with the Option<u8> type, for example, it's critical to note that a u8 can only represent values from 0 to 255. If you try to assign a value outside of this range, you'll encounter a compile-time error.



compile error가 난다고 함!

