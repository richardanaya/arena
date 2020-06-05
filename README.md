# slabify

Turn a struct into a slab real fast

```rust
#[slabify]
#[derive(Default,Debug)]
struct VideoFrame {
}

fn main() {
  let frame = VideoFrame::allocate();
  println!("{:?}",frame.load().lock().unwrap());
}
```
