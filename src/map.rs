//! Serializes an iterator of serializable 2-tuples into a serde map.
//!
//! This module is motsly identical to `serde_iter::seq`, except that iterators of tuples `(K, V)`
//! instead of single values are used.
//! Refer to the [`serde_iter::seq`](../seq/index.html) documentation for details.
//!
//! *This module requires the "map" feature to be enabled (enabled by default).*
//!
//! # Example
//! ```
//! #[derive(serde::Serialize)]
//! struct Foo {
//!     #[serde(with = "serde_iter::map")]
//!     bar: std::iter::Once<(&'static str, i32)>,
//! }
//!
//! let foo = Foo {
//!     bar: std::iter::once(("qux", 3)),
//! };
//! assert_eq!(serde_json::to_value(&foo).unwrap(), serde_json::json!({
//!     "bar": {
//!         "qux": 3
//!     }
//! }));
//! ```

use serde::ser::{Serialize, SerializeMap, Serializer};

/// Refer to the [module-level documentation](index.html).
pub fn serialize<S, T, K, V>(iter: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Iterator<Item = (K, V)> + Clone,
    K: Serialize,
    V: Serialize,
{
    let mut map = serializer.serialize_map(Some(iter.size_hint().0))?;
    for (key, value) in iter.clone() {
        map.serialize_entry(&key, &value)?;
    }
    map.end()
}

#[cfg(test)]
mod tests {
    use std::iter;

    use serde::Serialize;
    use serde_json::{json, to_value};

    #[derive(Serialize)]
    struct Foo<T>
    where
        T: Iterator<Item = (&'static str, usize)> + Clone,
    {
        #[serde(with = "super")]
        bar: T,
    }

    #[test]
    fn test_empty() {
        let value = to_value(Foo { bar: iter::empty() });
        let value = value.expect("Failed to serialize");
        assert_eq!(
            value,
            json!({
                "bar": {}
            })
        );
    }

    #[test]
    fn test_once() {
        let value = to_value(Foo {
            bar: iter::once(("qux", 3)),
        });
        let value = value.expect("Failed to serialize");
        assert_eq!(
            value,
            json!({
                "bar": {"qux": 3}
            })
        );
    }

    #[test]
    fn test_vec_map() {
        let vec = vec!["abcdef", "abcdefg"];
        let value = to_value(Foo {
            bar: vec.iter().map(|x| (*x, x.len())),
        });
        let value = value.expect("Failed to serialize");
        assert_eq!(
            value,
            json!({
                "bar": {
                    "abcdef": 6,
                    "abcdefg": 7
                }
            })
        );
    }
}
