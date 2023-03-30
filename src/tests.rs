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
}