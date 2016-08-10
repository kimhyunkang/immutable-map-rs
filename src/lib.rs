//! Immutable binary search tree.
//!
//! This crate provides functional programming style binary search trees which returns modified
//! copy of original map or set with the new data, and preserves the original. Many features and
//! algorithms are borrowed from `Data.Map` of Haskell's standard library.
//!
//! See https://yoichihirai.com/bst.pdf for the balancing algorithm.
//!
//! To share the data between the old and the new data structure after modification, most of the
//! functions require the key and value type to implement `Clone`. If you want to store non-
//! clonable data into this map, you can wrap it under shared pointer such as `Rc` or `Arc`.

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

/// An immutable set based on binary search tree
pub mod set;
/// An immutable map based on binary search tree
pub mod map;
mod tree;

pub use set::TreeSet;
pub use map::TreeMap;

/// An endpoint of a range of keys.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Bound<T> {
    /// An infinite endpoint. Indicates that there is no bound in this direction.
    Unbounded,
    /// An inclusive bound.
    Included(T),
    /// An exclusive bound.
    Excluded(T)
}

#[cfg(test)]
impl<T: Arbitrary> Arbitrary for Bound<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Bound<T> {
        match g.size() % 3 {
            0 => Bound::Unbounded,
            1 => Bound::Included(Arbitrary::arbitrary(g)),
            2 => Bound::Excluded(Arbitrary::arbitrary(g)),
            _ => panic!("remainder is greater than 3")
        }
    }
}
