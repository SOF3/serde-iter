use std::cell::Cell;

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
///     I: IntoIterator<Item = u32> + Clone,
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
///     I: IntoIterator<Item = u32> + Clone,
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
pub struct CloneOnce<T, I>(Cell<Option<I>>)
where
    I: IntoIterator<Item = T>;

/// Converts a (non-Clone) iterator into a CloneOnce iterator.
impl<T, I> From<I> for CloneOnce<T, I>
where
    I: IntoIterator<Item = T>,
{
    fn from(iter: I) -> Self {
        Self(Cell::new(Some(iter)))
    }
}

/// Moves the underlying iterator to a cloned value, and leaves a panicking iterator.
impl<T, I> Clone for CloneOnce<T, I>
where
    I: IntoIterator<Item = T>,
{
    #[inline]
    fn clone(&self) -> Self {
        let oi = self.0.take();
        if oi.is_none() {
            panic!("Attempt to clone a CloneOnce twice");
        }

        Self(Cell::new(oi))
    }
}

impl<T, I> IntoIterator for CloneOnce<T, I>
where
    I: IntoIterator<Item = T>,
{
    type Item = T;
    type IntoIter = <I as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.take().expect("Attempt to iterate over an empty CloneOnce").into_iter()
    }
}
