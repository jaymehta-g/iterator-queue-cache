#[cfg(test)]
mod tests {

use crate::*;
	#[test]
	fn queue_some() {
		let mut iter = vec![1,2,3,4,5].into_iter().cache(3).unwrap();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), None);
	}
	#[test]
	#[should_panic]
	fn queue_too_many() {
		let mut iter = vec![1,2,3,4,5].into_iter().cache(6).unwrap();
	}
	#[test]
	fn queue_all() {
		let mut iter = vec![1,2,3,4,5].into_iter().cache_all();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), None);
	}
	#[test]
	fn queue_overshoot_ok() {
		let mut iter = vec![1,2,3,4,5].into_iter().cache_or_all(500);
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), None);
	}
	#[test]
	fn iterator_methods() {
		let mut iter = vec![1,2,3,4,5].into_iter()
			.map(|i| i*2)
			.cache_all();
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), Some(6));
		assert_eq!(iter.next(), Some(8));
		assert_eq!(iter.next(), Some(10));
		assert_eq!(iter.next(), None);
	}
	#[test]
	fn lasy_eval() {
		static mut COUNTER: i32 = 0;
		let mut iter = vec![1,2,3,4,5].into_iter()
			.map(|i| {
				unsafe { // unsafe for threading reasons, probably ok
					COUNTER += 1;
				}
				i*2
			});
		unsafe {
			assert_eq!(COUNTER,0);
		}
		let mut iter = iter.cache(2).unwrap();
		unsafe {
			assert_eq!(COUNTER,2);
		}
		iter.cache_more(2);
		unsafe {
			assert_eq!(COUNTER,4);
			COUNTER = 0;
		}
		let mut iter = vec![1,2,3,4,5].into_iter()
			.map(|i| {
				unsafe { // unsafe for threading reasons, probably ok
					COUNTER += 1;
				}
				i*2
			})
			.cache_all();
		unsafe {
			assert_eq!(COUNTER,5);
		}
		
	}
	#[test]
	fn return_errors() {
		let err = (0..5).cache(40);
		match err {
			Ok(_) => panic!(),
			Err(_) => (),
		}
	}
	#[test]
	fn from_doc_test() {
		let mut cache = (0..5).cache_or_all(2); // Caches 2 values
     	cache.cache_more(2).unwrap(); // Caches 2 more values for a total of 4
    	cache.for_each(|i| println!("{}",i)); // Exhausts the 4 values in the cache, then prints the last item directly out of the iterator
	}
}