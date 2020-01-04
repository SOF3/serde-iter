use std::cell::RefCell;

/// A hack utility struct to wrap use-once iterators.
///
/// # Clone semantics
/// Every time CloneOnce is cloned, the underlying iterator is moved to the CloneOnce returned by
/// the `clone` method, and the original CloneOnce will panick if it is iterated over.
/// This behaviour would result in `serde_iter` consuming the underlying iterator exactly once if
/// it is only serialized exactly once.
///
/// # Usage
/// Wrap your iterator with this struct if you are **very sure** that it will only be serialized
/// once. This hack is necessary if your iterator does not implement `Clone`.
///
/// # Example
/// ```
/// #[derive(serde::Serialize)]
/// struct Foo<I>
/// where
///     I: Iterator<Item = u32> + Clone,
/// {
///     #[serde(with = "serde_iter::seq")]
///     bar: I,
/// }
///
/// let mut v = vec![1, 2, 3];
/// let drain = v.drain(..);
/// let foo = Foo {
///     bar: serde_iter::CloneOnce::from(drain),
/// };
///
/// assert_eq!(serde_json::to_value(&foo).unwrap(), serde_json::json!({
///     "bar": [1, 2, 3]
/// }));
/// ```
///
/// If this struct is serialized again, it panicks:
/// ```should_panic
/// #[derive(serde::Serialize)]
/// struct Foo<I>
/// where
///     I: Iterator<Item = u32> + Clone,
/// {
///     #[serde(with = "serde_iter::seq")]
///     bar: I,
/// }
///
/// let mut v = vec![1, 2, 3];
/// let drain = v.drain(..);
/// let foo = Foo {
///     bar: serde_iter::CloneOnce::from(drain),
/// };
///
/// assert_eq!(serde_json::to_value(&foo).unwrap(), serde_json::json!({
///     "bar": [1, 2, 3]
/// }));
/// serde_json::to_value(&foo).ok();
/// ```
pub struct CloneOnce<T, I>(RefCell<Option<I>>)
where
    I: Iterator<Item = T>;

/// Converts a (non-Clone) iterator into a CloneOnce iterator.
impl<T, I> From<I> for CloneOnce<T, I>
where
    I: Iterator<Item = T>,
{
    fn from(iter: I) -> Self {
        Self(RefCell::new(Some(iter)))
    }
}

/// Moves the underlying iterator to a cloned value, and leaves a panicking iterator.
impl<T, I> Clone for CloneOnce<T, I>
where
    I: Iterator<Item = T>,
{
    #[inline]
    fn clone(&self) -> Self {
        let mut borrow = self.0.borrow_mut();
        let oi = borrow.take();
        if oi.is_none() {
            panic!("Attempt to clone a CloneOnce twice");
        }
        drop(borrow);

        Self(RefCell::new(oi))
    }
}

impl<T, I> Iterator for CloneOnce<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let borrow = self.0.get_mut();
        let option = borrow.as_mut();
        let iter = option.expect("Attempt to iterate over a CloneOnce");
        let ret = iter.next();
        drop(borrow);

        ret
    }
}
