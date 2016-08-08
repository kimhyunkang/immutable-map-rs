#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

pub mod set;
pub mod map;
mod tree;

pub use set::Set;
pub use map::Map;

#[derive(Clone, Debug)]
pub enum Bound<T> {
    Unbounded,
    Included(T),
    Excluded(T)
}
