#[cfg(test)]
mod tests;

extern crate queues; 
use queues::*;
use std::{error::*, fmt::Display};
/**
 * Utility
 */
type BoxResult<T> = Result<T,Box<dyn Error>>;
/**
 * When `enqueue()` is called but the iterator is empty
 */
#[derive(Debug)]
pub struct EnqueueWhileIteratorIsEmptyError;
impl Display for EnqueueWhileIteratorIsEmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"`enqueue()` called but the iterator was empty!")
    }
}
impl Error for EnqueueWhileIteratorIsEmptyError {}
/**
 * An iterator that 'unlazies your evaluation' by caching items from your iterator now to be used later
 * Extract a certain amount of items from any iterator into a cache with the `queue()` method
 * When extracting from the resulting iterator, items will first be pulled from cache until it is exhausted 
 */
pub struct IteratorQueueCache<T: Clone>
where
    Queue<T>: Default, // Queue has trait bound for T
{
    queue: Queue<T>,
    iterator: Box<dyn Iterator<Item = T>>,
}
impl<T> IteratorQueueCache<T>
where
    T: Clone,
{
    pub fn new(iter: impl Iterator<Item = T> + 'static) -> Self {
        IteratorQueueCache {
            queue: Queue::default(),
            iterator: Box::new(iter),
        }
    }
    // methods
    /**
     * Lets the struct cache further items after its creation
     */
    pub fn enqueue(&mut self, quantity: u32) -> BoxResult<()> {
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
impl<T> Iterator for IteratorQueueCache<T>
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
pub trait IterateIntoQueue: Iterator
    where Self::Item: Clone 
{
    /**
     * Cache a given amount of items from this iterator for use later
     */
    fn queue(self, quantity: u32) -> BoxResult<IteratorQueueCache<Self::Item>>;
    /**
     * Cache all items from this iterator for use later
     */
    fn queue_all(self) -> IteratorQueueCache<Self::Item>;
    /**
     * Cache a given amount of items from this iterator for use later, or end early if the iterator runs out
     */
    fn queue_or_all(self, quantity: u32) -> IteratorQueueCache<Self::Item>;
}
impl<'a,T> IterateIntoQueue for T
    where T: Iterator + 'static,
        T::Item: Clone
{
    fn queue(self, quantity: u32) -> BoxResult<IteratorQueueCache<Self::Item>> {
        let mut cache = IteratorQueueCache::new(self);
        cache.enqueue(quantity)?;
        Ok(cache)
    }

    fn queue_all(self) -> IteratorQueueCache<Self::Item> {
        let mut cache = IteratorQueueCache::new(self);
        loop {
            if let Err(_) = cache.enqueue(1) { break; }
        }
        cache
    }

    fn queue_or_all(self, quantity: u32) -> IteratorQueueCache<Self::Item> {
        let mut cache = IteratorQueueCache::new(self);
        for _ in 0..quantity {
            if let Err(_) = cache.enqueue(1) { break; }
        }
        cache
    }
}

