use iterator_queue_cache::*;
fn main() {
	let vec = vec![1,2,3,4,5];
	let iter = vec.into_iter().queue(3).unwrap();
	iter.for_each(|i| println!("{}",i))
}