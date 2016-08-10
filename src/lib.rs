#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

pub mod set;
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
