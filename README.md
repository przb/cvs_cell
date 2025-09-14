# `CvsCell`


Similar to a [`Cell`](core::cell::Cell), but with a few major differences.
- [`CvsCell`] implements both [`Send`] **and** [`Sync`].
- The constructor, [`new`](CvsCell::new), is unsafe.
- All reads and writes are volatile.


## Examples

```rust
use cvs_cell::CvsCell;

static C: CvsCell<u32> = unsafe { CvsCell::new(0) };

fn main() {
  assert_eq!(C.get(), 0);
}
```
