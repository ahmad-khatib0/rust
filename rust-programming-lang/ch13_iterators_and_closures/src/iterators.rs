// In Rust, iterators are lazy, meaning they have no effect until you
// call methods that consume the iterator to use it up.

pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // The Iterator Trait and the next Method
}

#[cfg(test)]
mod tests {
    // ANCHOR: here
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // If we want to create an iterator that takes ownership of v1 and returns owned values,
        // we can call into_iter instead of iter. Similarly, if we want to iterate over mutable
        // references, we can call iter_mut instead of iter
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    // ANCHOR_END: here
    // Note that we needed to make v1_iter mutable: calling the next method on an iterator changes
    // internal state that the iterator uses to keep track of where it is in the sequence. In other
    // words, this code consumes, or uses up, the iterator. Each call to next eats up an item from
    // the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop
    // took ownership of v1_iter and made it mutable behind the scenes.

    // Methods That Consume the Iterator
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        // not allowed to use v1_iter after the call to sum because sum takes ownership of the iterator
        assert_eq!(total, 6);
    }
}
