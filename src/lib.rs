//! # Iterator Cache
//! 'Unlazies your evaluation' by caching items from your iterator now to be used later. 
//! 
//! Could be useful when an iterator is running an expensive operation and you'd rather it run now and store the values for later than perform the operation later.
#[cfg(test)]
#[allow(rustdoc::invalid_codeblock_attributes)]
mod tests;

extern crate queues; 
use queues::*;
use std::{error::*, fmt::Display};
/// Utility
type BoxResult<T> = Result<T,Box<dyn Error>>;
/// Error thrown when `enqueue()` is called but the iterator is empty
#[derive(Debug)]
pub struct EnqueueWhileIteratorIsEmptyError;
impl Display for EnqueueWhileIteratorIsEmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"`cache` or `cache_more` called but the iterator was empty!")
    }
}
impl Error for EnqueueWhileIteratorIsEmptyError {}
/// Extract a certain amount of items from any iterator into a cache with the `cache()` method.
/// When extracting from the resulting iterator, items will first be pulled from cache until it is exhausted .
pub struct IteratorCache<T: Clone>
where
    Queue<T>: Default, // Queue has trait bound for T
{
    queue: Queue<T>,
    iterator: Box<dyn Iterator<Item = T>>,
}
impl<T> IteratorCache<T>
where
    T: Clone,
{
    /// Create a new cache with an iterator.
    /// Identical to `iterator.queue(0).unwrap()` or `iterator.queue_or_all(0)` 
    pub fn new(iter: impl Iterator<Item = T> + 'static) -> Self {
        IteratorCache {
            queue: Queue::default(),
            iterator: Box::new(iter),
        }
    }
    // methods
    /// Caches more items from the contained iterator.
    /// # Example:
    /// ```ignore
    /// let mut cache = (0..5).cache_or_all(2); // Caches 2 values
    /// cache.cache_more(2); // Caches 2 more values for a total of 4
    /// cache.for_each(|i| println!("{}",i)); // Exhausts the 4 values in the cache, then prints the last item directly out of the iterator
    pub fn cache_more(&mut self, quantity: u32) -> BoxResult<()> {
        for _ in 0..quantity {
            if let Some(item) = self.iterator.next() {
                self.queue.add(item)?;
            } else {
                return Err(Box::new(EnqueueWhileIteratorIsEmptyError {}));
            }
        }
        Ok(())
    }
}
/**
 * Take cached values from queue if available, else take from iterator
 */
impl<T> Iterator for IteratorCache<T>
where
    T: Clone,
{
    type Item = T;
    /**
     * Returns cached items if available, otherwise takes items from the owned iterator
     */
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(item) = self.queue.remove() {
            return Some(item);
        }
        return self.iterator.next();
    }
}
/// Any Iterator can have its values cached with the methods in this trait.
/// (Currently only iterators where Item: Clone are supported, due to trait bounds on the queue from the `queues` crate)
pub trait IterateIntoCache: Iterator
    where Self::Item: Clone 
{
    /// Cache a given amount of items from this iterator for use later.
    /// # Errors
    /// Errors if the iterator runs out of items while caching
    /// To avoid the error, consider using `cache_or_all()`
    fn cache(self, quantity: u32) -> BoxResult<IteratorCache<Self::Item>>;
    /**
     * Cache all items from this iterator for use later
     */
    fn cache_all(self) -> IteratorCache<Self::Item>;
    /**
     * Cache a given amount of items from this iterator for use later, or end early if the iterator runs out
     */
    fn cache_or_all(self, quantity: u32) -> IteratorCache<Self::Item>;
}
impl<'a,T> IterateIntoCache for T
    where T: Iterator + 'static,
        T::Item: Clone
{
    fn cache(self, quantity: u32) -> BoxResult<IteratorCache<Self::Item>> {
        let mut cache = IteratorCache::new(self);
        cache.cache_more(quantity)?;
        Ok(cache)
    }

    fn cache_all(self) -> IteratorCache<Self::Item> {
        let mut cache = IteratorCache::new(self);
        loop {
            if let Err(_) = cache.cache_more(1) { break; }
        }
        cache
    }

    fn cache_or_all(self, quantity: u32) -> IteratorCache<Self::Item> {
        let mut cache = IteratorCache::new(self);
        for _ in 0..quantity {
            if let Err(_) = cache.cache_more(1) { break; }
        }
        cache
    }
}

