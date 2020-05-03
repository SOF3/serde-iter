//! Serializes an iterator of serializables into a serde sequence.
//!
//! *This module requires the "seq" feature to be enabled (enabled by default).*
//!
//! # Usage
//! To derive `Serialize` for a struct that contains arbitrary `Iterator` types,
//! simply add `#[serde(with = "serde_iter::seq")]` on the fields using such types.
//!
//! # Example
//! ```
//! #[derive(serde::Serialize)]
//! struct Foo {
//!     #[serde(with = "serde_iter::seq")]
//!     bar: std::iter::Once<i32>,
//! }
//!
//! let foo = Foo {
//!     bar: std::iter::once(2),
//! };
//! assert_eq!(serde_json::to_value(&foo).unwrap(), serde_json::json!({
//!     "bar": [2]
//! }));
//! ```
//!
//! # Cloning
//! Since serialization could be called multiple times on the same value,
//! each time the iterator is serialized, `serde_iter` would clone the iterator and only consume
//! the clone.
//! As a result, the Iterator type must implement `Clone`, which is not implemented by all
//! `Iterator` types.
//!
//! For example, `std::vec::Drain` does not implement `Clone`, because consuming the iterator
//! modifies the `Vec`, and doing so multiple times would lead to different semantics:
//!
//! ```compile_fail
//! #[derive(serde::Serialize)]
//! struct Foo<'a> {
//!     #[serde(with = "serde_iter::seq")]
//!     bar: std::vec::Drain<'a, i32>,
//! }
//!
//! let mut v = vec![];
//! serde_json::to_value(&Foo { bar: v.drain(..) }).unwrap();;
//! ```
//!
//! This fails to compile, because if the `serde_json::to_value` is called twice, unexpected
//! behaviour would occur: should it yield inconsistent results, or otherwise store the drained
//! data somewhere else?
//!
//! Furthermore, since cloning is not cost-free, if cloning the iterator involves cloning the
//! iterated object, it is *advised* that the iterated type be a `Copy`,
//! or at least some low-cost `Clone` (and low-cost `Drop`).
//! For example, `Rc::clone()`/`Rc::drop()` only increments/decrements a refcount value, so it is
//! relatively cheap.
//! On the other hand, `String::clone()` allocates a new buffer on the heap and copies all
//! characters to the new buffer, which is not low-cost, and it is better to borrow the strings.
//!
//! Consider this example:
//!
//! ```
//! # use std::sync::atomic::{AtomicU8, Ordering};
//! # use serde::Serialize;
//! #
//! static counter: AtomicU8 = AtomicU8::new(0);
//!
//! fn do_something_costly() {
//!     counter.fetch_add(1, Ordering::Relaxed);
//! }
//!
//! #[derive(serde::Serialize)]
//! struct Costly;
//!
//! impl Clone for Costly {
//!     fn clone(&self) -> Self {
//!         do_something_costly();
//!         Self
//!     }
//! }
//!
//! #[derive(serde::Serialize)]
//! struct Foo<T: Iterator<Item = V> + Clone, V: Serialize> {
//!     #[serde(with = "serde_iter::seq")]
//!     bar: T,
//! }
//!
//! let v = vec![Costly]; // we just have one Costly item
//! let foo = Foo { bar: v.into_iter() };
//! assert_eq!(counter.load(Ordering::Relaxed), 0);
//! serde_json::to_string(&foo).unwrap();
//! assert_eq!(counter.load(Ordering::Relaxed), 1);
//! serde_json::to_string(&foo).unwrap();
//! assert_eq!(counter.load(Ordering::Relaxed), 2);
//! ```
//!
//! The `Costly` type gets cloned once on every serialization.
//! This is because each serialization clones `Costly`.
//! This may result in significant differences for non-`Copy` types,
//! such as `String`.
//!
//! On the other hand, if the iterator only contains references ot the cloned type:
//! ```
//! # use std::sync::atomic::{AtomicU8, Ordering};
//! # use serde::Serialize;
//! #
//! # static counter: AtomicU8 = AtomicU8::new(0);
//! #
//! # fn do_something_costly() {
//! #     counter.fetch_add(1, Ordering::Relaxed);
//! # }
//! #
//! # #[derive(serde::Serialize)]
//! # struct Costly;
//! #
//! # impl Clone for Costly {
//! #     fn clone(&self) -> Self {
//! #         do_something_costly();
//! #         Self
//! #     }
//! # }
//! #
//! # #[derive(serde::Serialize)]
//! # struct Foo<T: Iterator<Item = V> + Clone, V: Serialize> {
//! #     #[serde(with = "serde_iter::seq")]
//! #     bar: T,
//! # }
//! #
//! // Previous definitions unchanged
//!
//! let v = vec![Costly]; // we just have one Costly item
//! let foo = Foo { bar: v.iter() };
//! assert_eq!(counter.load(Ordering::Relaxed), 0);
//! serde_json::to_string(&foo).unwrap();
//! assert_eq!(counter.load(Ordering::Relaxed), 0);
//! serde_json::to_string(&foo).unwrap();
//! assert_eq!(counter.load(Ordering::Relaxed), 0);
//! ```
//!
//! Since `&v` only iterates `&Costly` but not `Costly`, no clones are performed.
//!
//! Note that this this distinction mainly applies for pre-existing values.
//! If the clonable value is created during iteration,
//! e.g. if it is `iter.map(|x| x.to_string())`,
//! borrowing would not improve anything;
//! to prevent cloning unnecessarily, it might be desirable to
//! store the mapped data in a `Vec` beforehand.

use serde::ser::{Serialize, SerializeSeq, Serializer};

/// Refer to the [module-level documentation](index.html).
pub fn serialize<S, T, V>(iter: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: IntoIterator<Item = V> + Clone,
    V: Serialize,
{
    let iter = iter.clone().into_iter();
    let mut seq = serializer.serialize_seq(Some(iter.size_hint().0))?;
    for value in iter {
        seq.serialize_element(&value)?;
    }
    seq.end()
}

#[cfg(test)]
mod tests {
    use std::iter;

    use serde::Serialize;
    use serde_json::{json, to_value};

    #[derive(Serialize)]
    struct Foo<T>
    where
        T: Iterator<Item = i32> + Clone,
    {
        #[serde(with = "super")]
        bar: T,
    }

    #[test]
    fn test_once() {
        let value = to_value(Foo { bar: iter::once(2) });
        let value = value.expect("Failed to serialize");
        assert_eq!(
            value,
            json!({
                "bar": [2]
            })
        );
    }

    #[test]
    fn test_empty() {
        let value = to_value(Foo { bar: iter::empty() });
        let value = value.expect("Failed to serialize");
        assert_eq!(
            value,
            json!({
                "bar": []
            })
        );
    }
}
