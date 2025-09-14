[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[`Sync`]: https://doc.rust-lang.org/core/marker/trait.Sync.html
[`Send`]: https://doc.rust-lang.org/core/marker/trait.Send.html


# `CvsCell`


Similar to a [`Cell`](core::cell::Cell), but with a few major differences.
- [`CvsCell`] implements both [`Send`] **and** [`Sync`].
- The constructor, [`new`](CvsCell::new), is unsafe.
- All reads and writes are volatile.

This is designed to be used in a single-threaded environment, though it can be used with
multiple threads if the programmer verifies that a reference is not shared between multiple
threads.


## Safety


This [`CvsCell`] cannot be shared between multiple threads. This is must be validated
by the user of the [`CvsCell`], hence why [`new`](CvsCell::new) is marked as unsafe.



## Examples

### Intended Usage
```rust
use cvs_cell::CvsCell;

static C: CvsCell<u32> = unsafe { CvsCell::new(0) };

fn main() {
  assert_eq!(C.get(), 0);
}
```

### Unsafe Usage
This is a bit of a contrived example, but this would cause race conditions, as the
the cell is being shared between threads
```rust,no_run
use cvs_cell::CvsCell;

static C: CvsCell<u32> = unsafe { CvsCell::new(0) };

fn increment_1000() {
  for i in 0..1000 {
    C.update(|old_val| old_val + 1);
  }
}

fn main() {
  // Using a thread scope so we don't need to manually join them all at the end
  let jh = std::thread::scope(|s|{
    // Spawn 10 threads
    for i in 0..10 {
      s.spawn(increment_1000);
    }
  });

  // This may not equal 10,000!
  assert_eq!(C.get(), 10 * 1000)
}

```
