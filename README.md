# iterator-queue-cache
Evaluate iterators now rather than later

Gives iterators the ability to evaluate and store their values now, to be drawn from afterwards
# Example
```rust
use iterator_cache::*;

fn main() {
    let mut iter = (0..10)
        .map(|i| {
            println!("called with value {i}!");
            i
        }); // closure does not get called yet
    let mut cache_iter = iter.cache(5).unwrap(); // .next() is called 5 times here
    cache_iter.for_each(|i| {
        println!("afterwards called with value {i}")
    }) // the first 5 values are taken from the cache, then the rest are taken from the iterator
    // outputs:
//     called with value 0!             // start storing the values into cache
//     called with value 1!
//     called with value 2!
//     called with value 3!
//     called with value 4!
//     afterwards called with value 0   // remove all values from the cache
//     afterwards called with value 1
//     afterwards called with value 2
//     afterwards called with value 3
//     afterwards called with value 4
//     called with value 5!             // cache is exhausted, take from iterator
//     afterwards called with value 5
//     called with value 6!
//     afterwards called with value 6
//     called with value 7!
//     afterwards called with value 7
//     called with value 8!
//     afterwards called with value 8
//     called with value 9!
//     afterwards called with value 9
}
```
See [docs](https://docs.rs/iterator-cache/0.1.0/iterator_cache/) for more info
