#[cfg(test)]
mod tests {
	use crate::*;
	#[test]
	fn queue_some() {
		let mut iter = vec![1,2,3,4,5].into_iter().queue(3).unwrap();
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
		let mut iter = vec![1,2,3,4,5].into_iter().queue(6).unwrap();
	}
	#[test]
	fn queue_all() {
		let mut iter = vec![1,2,3,4,5].into_iter().queue_all();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), None);
	}
	#[test]
	fn queue_overshoot_ok() {
		let mut iter = vec![1,2,3,4,5].into_iter().queue_or_all(500);
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
			.queue_all();
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
		let mut iter = iter.queue(2).unwrap();
		unsafe {
			assert_eq!(COUNTER,2);
		}
		iter.enqueue(2);
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
			.queue_all();
		unsafe {
			assert_eq!(COUNTER,5);
		}
		
	}
	#[test]
	fn return_errors() {
		let err = (0..5).queue(40);
		match err {
			Ok(_) => panic!(),
			Err(_) => (),
		}
	}
}