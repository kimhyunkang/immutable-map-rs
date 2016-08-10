use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::Index;
use std::rc::Rc;

use Bound;

use tree;
use tree::TreeNode;

/// An immutable key-value map based on weight-balanced binary tree.
/// See https://yoichihirai.com/bst.pdf for the balancing algorithm.
///
/// # Examples
///
/// ```
/// use immutable_map::TreeMap;
///
/// let map_0 = TreeMap::new();
///
/// // `insert` returns new copies with the given key and value inserted, and does not change
/// // the original map
/// let map_1 = map_0.insert(3, "Three");
/// let map_2 = map_1.insert(4, "Four");
///
/// assert_eq!(false, map_1.contains_key(&4));
/// assert_eq!(true, map_2.contains_key(&4));
///
/// assert_eq!("Four", map_2[&4]);
/// ```
#[derive(Clone, Default)]
pub struct TreeMap<K, V> {
    root: Option<Rc<TreeNode<K, V>>>
}

pub type TreeMapIter<'r, K, V> = tree::Iter<'r, K, V>;
pub type TreeMapRevIter<'r, K, V> = tree::RevIter<'r, K, V>;
pub type TreeMapRange<'r, K, V> = tree::Range<'r, K, V>;

impl<K, V> TreeMap<K, V> {
    /// Makes a new empty TreeMap
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new();
    /// let new_map = map.insert("One", 1);
    /// ```
    pub fn new() -> TreeMap<K, V> {
        TreeMap { root: None }
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(1, "One").insert(2, "Two");
    /// assert_eq!(2, map.len());
    /// ```
    pub fn len(&self) -> usize {
        tree::size(&self.root)
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let empty_map = TreeMap::new();
    /// let new_map = empty_map.insert(1, "One");
    ///
    /// assert_eq!(true, empty_map.is_empty());
    /// assert_eq!(false, new_map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Gets an iterator over the entries of the map, sorted by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// for (key, value) in map.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = map.iter().next().unwrap();
    /// assert_eq!((1, "One"), (*first_key, *first_value));
    /// ```
    pub fn iter<'r>(&'r self) -> TreeMapIter<'r, K, V> {
        tree::Iter::new(&self.root)
    }

    /// Gets an iterator over the entries of the map, sorted by key in decreasing order.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// for (key, value) in map.rev_iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = map.rev_iter().next().unwrap();
    /// assert_eq!((3, "Three"), (*first_key, *first_value));
    /// ```
    pub fn rev_iter<'r>(&'r self) -> TreeMapRevIter<'r, K, V> {
        tree::RevIter::new(&self.root)
    }

    /// Gets an iterator over the keys of the map, in increasing order.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    ///
    /// let first_key = map.keys().next().unwrap();
    /// assert_eq!(1, *first_key);
    /// ```
    pub fn keys<'r>(&'r self) -> tree::Keys<TreeMapIter<'r, K, V>> {
        tree::Keys::new(tree::Iter::new(&self.root))
    }

    /// Gets an iterator over the values of the map, ordered by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// for value in map.values() {
    ///     println!("{}", value);
    /// }
    ///
    /// let first_value = map.values().next().unwrap();
    /// assert_eq!("One", *first_value);
    /// ```
    pub fn values<'r>(&'r self) -> tree::Values<TreeMapIter<'r, K, V>> {
        tree::Values::new(tree::Iter::new(&self.root))
    }
}

impl<K, V> TreeMap<K, V> where K: Ord {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering on the borrowed
    /// form must match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(1, "One");
    ///
    /// assert_eq!(map.get(&1), Some(&"One"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get<Q: ?Sized + Ord>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>
    {
        fn f<'r, K, V, Q: ?Sized + Ord>(node: &'r Option<Rc<TreeNode<K, V>>>, key: &Q)
                -> Option<&'r V> where K: Borrow<Q>
        {
            tree::find_exact(node, |k| key.cmp(k.borrow())).map(|p| &p.1)
        }

        f(&self.root, key)
    }

    /// Returns true if the map contains given key
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering on the borrowed
    /// form must match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(1, "One");
    ///
    /// assert_eq!(true, map.contains_key(&1));
    /// assert_eq!(false, map.contains_key(&2));
    /// ```
    pub fn contains_key<Q: ?Sized + Ord>(&self, key: &Q) -> bool
        where K: Borrow<Q>
    {
        self.get(key).is_some()
    }

    /// Constructs a double-ended iterator over a sub-range of elements in the map, starting at
    /// min, and ending at max. If min is Unbounded, then it will be treated as "negative
    /// infinity", and if max is Unbounded, then it will be treated as "positive infinity". Thus
    /// range(Unbounded, Unbounded) will yield the whole collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    /// use immutable_map::Bound::*;
    ///
    /// let map = TreeMap::new().insert(8, "Eight").insert(3, "Three").insert(5, "Five");
    ///
    /// for (key, value) in map.range(Included(&4), Included(&8)) {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let pairs: Vec<_> = map.range(Included(&4), Included(&8)).map(|(k, v)| (*k, *v)).collect();
    ///
    /// assert_eq!(pairs, [(5, "Five"), (8, "Eight")]);
    /// ```
    pub fn range<'r, Q: Ord>(&'r self, min: Bound<&Q>, max: Bound<&Q>) -> TreeMapRange<'r, K, V>
        where K: Borrow<Q>
    {
        tree::Range::new(&self.root, min, max)
    }
}

impl<K, V> TreeMap<K, V> where K: Clone + Ord, V: Clone {
    /// Return a new copy of `TreeMap` with the key-value pair inserted
    ///
    /// If the map already has the key, the key-value pair is replaced in the new map
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new();
    ///
    /// assert_eq!(false, map.contains_key(&1));
    /// assert_eq!(None, map.get(&1));
    ///
    /// let new_map = map.insert(1, "One");
    ///
    /// assert_eq!(true, new_map.contains_key(&1));
    /// assert_eq!(Some(&"One"), new_map.get(&1));
    /// ```
    pub fn insert(&self, key: K, value: V) -> TreeMap<K, V>
    {
        let root = tree::insert(&self.root, (key, value));
        TreeMap { root: Some(Rc::new(root)) }
    }

    /// Return a new copy of `TreeMap` with the key-value pair inserted.
    ///
    /// Returns `None` if the map already has the key
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert(2, "Two").insert(3, "Three");
    ///
    /// assert_eq!(None, map.insert_if_absent(2, "Zwei"));
    ///
    /// let new_map = map.insert_if_absent(1, "One").unwrap();
    ///
    /// assert_eq!(Some(&"One"), new_map.get(&1));
    /// ```
    pub fn insert_if_absent(&self, key: K, value: V) -> Option<TreeMap<K, V>>
    {
        tree::insert_if_absent(&self.root, (key, value)).map(|root|
            TreeMap { root: Some(Rc::new(root)) }
        )
    }

    /// Find the map with given key, and if the key is found, udpate the value with the provided
    /// function `f`, and return the new map. Returns `None` if the map already has the key.
    ///
    /// When the key is found in the map, function `f` is called, and the value is updated with
    /// the return value of `f`.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering on the borrowed
    /// form must match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert("Two".to_string(), 2).insert("Three".to_string(), 3);
    ///
    /// // returns `None` because the key "One" is not in the map
    /// assert_eq!(None, map.update("One", |v| v+1));
    ///
    /// let map_1 = map.update("Two", |v| v+10).unwrap();
    /// // the value is updated
    /// assert_eq!(Some(&12), map_1.get("Two"));
    /// ```
    pub fn update<Q: ?Sized + Ord, F>(&self, key: &Q, f: F) -> Option<TreeMap<K, V>>
        where K: Borrow<Q>, F: FnMut(&V) -> V
    {
        match self.root {
            Some(ref root) =>
                tree::update(root, key, f).map(|new_root|
                    TreeMap { root: Some(Rc::new(new_root)) }
                ),
            None =>
                None
        }
    }

    /// Find the map with given key, and if the key is found, udpate the value with the provided
    /// function `f`, and return the new map. If the key is not found, insert the key-value pair
    /// to the map and return it.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let map = TreeMap::new().insert("One", 1).insert("Three", 3);
    ///
    /// // The new pair ("Two", 2) is inserted
    /// let map_1 = map.insert_or_update("Two", 2, |v| v + 10);
    /// assert_eq!(Some(&2), map_1.get("Two"));
    ///
    /// // The ("Two", 2) pair is updated to ("Two", 2 + 10)
    /// let map_2 = map_1.insert_or_update("Two", 2, |v| v + 10);
    /// assert_eq!(Some(&12), map_2.get("Two"));
    /// ```
    pub fn insert_or_update<F>(&self, key: K, value: V, f: F) -> TreeMap<K, V>
        where F: FnMut(&V) -> V
    {
        TreeMap { root: Some(Rc::new(tree::insert_or_update(&self.root, key, value, f))) }
    }

    /// Remove the smallest key-value pair from the map, and returns the modified copy.
    ///
    /// Returns `None` if the original map was empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let empty_map = TreeMap::new();
    /// assert_eq!(None, empty_map.delete_min());
    ///
    /// let map = empty_map.insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// let (new_map, pair) = map.delete_min().unwrap();
    ///
    /// assert_eq!(None, new_map.get(&1));
    /// assert_eq!((&1, &"One"), pair);
    /// ```
    pub fn delete_min(&self) -> Option<(TreeMap<K, V>, (&K, &V))>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_min(&root);
            Some((
                TreeMap { root: new_root },
                (&v.0, &v.1)
            ))
        } else {
            None
        }
    }

    /// Remove the largest key-value pair from the map, and returns the modified copy.
    ///
    /// Returns `None` if the original map was empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let empty_map = TreeMap::new();
    /// assert_eq!(None, empty_map.delete_max());
    ///
    /// let map = empty_map.insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// let (new_map, pair) = map.delete_max().unwrap();
    ///
    /// assert_eq!(None, new_map.get(&3));
    /// assert_eq!((&3, &"Three"), pair);
    /// ```
    pub fn delete_max(&self) -> Option<(TreeMap<K, V>, (&K, &V))>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_max(&root);
            Some((
                TreeMap { root: new_root },
                (&v.0, &v.1)
            ))
        } else {
            None
        }
    }

    /// Remove the key from the map
    ///
    /// Returns `None` if the original map did not contain the key
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering on the borrowed
    /// form must match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use immutable_map::TreeMap;
    ///
    /// let empty_map = TreeMap::new();
    /// assert_eq!(None, empty_map.remove(&2));
    ///
    /// let map = empty_map.insert(2, "Two").insert(3, "Three").insert(1, "One");
    ///
    /// let (new_map, pair) = map.remove(&2).unwrap();
    ///
    /// assert_eq!(None, new_map.get(&2));
    /// assert_eq!(&"Two", pair);
    /// ```
    pub fn remove<Q: ?Sized + Ord>(&self, key: &Q) -> Option<(TreeMap<K, V>, &V)>
        where K: Borrow<Q>
    {
        tree::remove(&self.root, key).map(|(new_root, v)|
            (TreeMap { root: new_root }, &v.1)
        )
    }
}

impl<K: Debug + Ord, V: Debug> Debug for TreeMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'r, K: Ord, V> IntoIterator for &'r TreeMap<K, V> {
    type Item = (&'r K, &'r V);
    type IntoIter = TreeMapIter<'r, K, V>;

    fn into_iter(self) -> TreeMapIter<'r, K, V> {
        self.iter()
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for TreeMap<K, V> {
    fn eq(&self, other: &TreeMap<K, V>) -> bool {
        self.len() == other.len()
            && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<K: Eq, V: Eq> Eq for TreeMap<K, V> {}

impl <K: PartialOrd, V: PartialOrd> PartialOrd for TreeMap<K, V> {
    fn partial_cmp(&self, other: &TreeMap<K, V>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl <K: Ord, V: Ord> Ord for TreeMap<K, V> {
    fn cmp(&self, other: &TreeMap<K, V>) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <'a, K: Ord, Q: ?Sized, V> Index<&'a Q> for TreeMap<K, V>
    where K: Borrow<Q>, Q: Ord
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl <K: Ord + Clone, V: Clone> FromIterator<(K, V)> for TreeMap<K, V> {
    fn from_iter<T>(iter: T) -> TreeMap<K, V> where T: IntoIterator<Item=(K, V)> {
        let mut m = TreeMap::new();
        for (k, v) in iter {
            m = m.insert(k, v);
        }
        m
    }
}

#[cfg(test)]
mod test {
    use tree::balanced;

    use super::TreeMap;
    use Bound;

    #[test]
    fn test_insert() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let r7 = r6.insert(14, 'n');
        let r8 = r7.insert(18, 'r');
        let r9 = r8.insert(16, 'p');
        let r10 = r9.insert(17, 'q');

        let expected = vec![
            (3, 'c'), (4, 'd'), (5, 'e'), (7, 'g'),
            (12, 'l'), (14, 'n'), (15, 'o'), (16, 'p'),
            (17, 'q'), (18, 'r')
        ];

        let res: Vec<_> = r10.iter().map(|(&k, &v)| (k, v)).collect();

        assert_eq!(expected, res);
        assert_eq!(10, r10.len());
        assert!(balanced(&r10.root));
        assert!(r10.contains_key(&12));
    }

    #[test]
    fn test_delete_min() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let (r7, v) = r6.delete_min().unwrap();

        let expected = vec![(4, 'd'), (5, 'e'), (7, 'g'), (12, 'l'), (15, 'o')];
        let res: Vec<_> = r7.iter().map(|(&k, &v)| (k, v)).collect();

        assert_eq!(expected, res);
        assert_eq!((&3, &'c'), v);
    }

    #[test]
    fn test_delete_max() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let (r7, v) = r6.delete_max().unwrap();

        let expected = vec![(3, 'c'), (4, 'd'), (5, 'e'), (7, 'g'), (12, 'l')];
        let res: Vec<_> = r7.iter().map(|(&k, &v)| (k, v)).collect();

        assert_eq!(expected, res);
        assert_eq!((&15, &'o'), v);
    }

    #[test]
    fn test_remove() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let (r7, v) = r6.remove(&7).unwrap();

        let expected = vec![(3, 'c'), (4, 'd'), (5, 'e'), (12, 'l'), (15, 'o')];
        let res: Vec<_> = r7.iter().map(|(&k, &v)| (k, v)).collect();

        assert_eq!(expected, res);
        assert_eq!(&'g', v);
    }

    #[test]
    fn test_iter() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let r7 = r6.insert(14, 'n');
        let r8 = r7.insert(18, 'r');
        let r9 = r8.insert(16, 'p');
        let r10 = r9.insert(17, 'q');

        let expected = vec![
            (3, 'c'), (4, 'd'), (5, 'e'), (7, 'g'),
            (12, 'l'), (14, 'n'), (15, 'o'), (16, 'p'),
            (17, 'q'), (18, 'r')
        ];

        let res: Vec<_> = r10.iter().map(|(&k, &v)| (k, v)).collect();

        assert_eq!(expected, res);

        assert_eq!((10, Some(10)), r10.iter().size_hint());
    }

    #[test]
    fn test_rev_iter() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let r7 = r6.insert(14, 'n');
        let r8 = r7.insert(18, 'r');
        let r9 = r8.insert(16, 'p');
        let r10 = r9.insert(17, 'q');

        let expected = vec![
            (18, 'r'), (17, 'q'),
            (16, 'p'), (15, 'o'), (14, 'n'), (12, 'l'),
            (7, 'g'), (5, 'e'), (4, 'd'), (3, 'c')
        ];

        let res: Vec<_> = r10.rev_iter().map(|(&k, &v)| (k, v)).collect();

        assert_eq!(expected, res);

        assert_eq!((10, Some(10)), r10.rev_iter().size_hint());
    }

    #[test]
    fn test_is_empty() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');

        assert!(r0.is_empty());
        assert!(!r1.is_empty());
        assert!(!r2.is_empty());
    }

    #[test]
    fn test_range() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let r7 = r6.insert(14, 'n');
        let r8 = r7.insert(18, 'r');
        let r9 = r8.insert(16, 'p');
        let r10 = r9.insert(17, 'q');

        let expected = vec![(7, 'g'), (12, 'l'), (14, 'n'), (15, 'o'), (16, 'p')];

        let res: Vec<_> = r10.range(Bound::Included(&6), Bound::Excluded(&17))
                             .map(|(&k, &v)| (k, v))
                             .collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_range_rev() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');
        let r3 = r2.insert(12, 'l');
        let r4 = r3.insert(15, 'o');
        let r5 = r4.insert(3, 'c');
        let r6 = r5.insert(5, 'e');
        let r7 = r6.insert(14, 'n');
        let r8 = r7.insert(18, 'r');
        let r9 = r8.insert(16, 'p');
        let r10 = r9.insert(17, 'q');

        let expected = vec![(16, 'p'), (15, 'o'), (14, 'n'), (12, 'l'), (7, 'g')];

        let res: Vec<_> = r10.range(Bound::Included(&6), Bound::Excluded(&17))
                             .rev()
                             .map(|(&k, &v)| (k, v))
                             .collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_debug() {
        let r0 = TreeMap::new();
        let r1 = r0.insert(7, 'g');
        let r2 = r1.insert(4, 'd');

        assert_eq!("{4: 'd', 7: 'g'}", &format!("{:?}", r2));
    }
}

#[cfg(test)]
mod quickcheck {
    use map::TreeMap;
    use Bound;

    use quickcheck::TestResult;
    use rand::{Rng, StdRng};

    fn filter_input<K: PartialEq, V>(input: Vec<(K, V)>) -> Vec<(K, V)> {
        let mut res: Vec<(K, V)> = Vec::new();

        for (k, v) in input {
            if res.iter().all(|pair| pair.0 != k) {
                res.push((k, v));
            }
        }

        res
    }

    quickcheck! {
        fn check_length(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            m.len() == input.len()
        }
    }

    quickcheck! {
        fn check_is_empty(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            m.is_empty() == input.is_empty()
        }
    }

    quickcheck! {
        fn check_iter(xs: Vec<(isize, char)>) -> bool {
            let mut input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            input.sort();

            let collected: Vec<(isize, char)> = m.iter().map(|(&k, &v)| (k, v)).collect();

            collected == input
        }
    }

    quickcheck! {
        fn check_iter_size_hint(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            let n = m.len();
            m.iter().size_hint() == (n, Some(n))
        }
    }

    quickcheck! {
        fn check_rev_iter(xs: Vec<(isize, char)>) -> bool {
            let mut input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            input.sort();
            input.reverse();

            let collected: Vec<(isize, char)> = m.rev_iter().map(|(&k, &v)| (k, v)).collect();

            collected == input
        }
    }

    quickcheck! {
        fn check_get(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            input.into_iter().all(|(k, v)| m.get(&k) == Some(&v))
        }
    }

    quickcheck! {
        fn check_remove(xs: Vec<(isize, char)>) -> TestResult {
            if xs.is_empty() {
                return TestResult::discard();
            }

            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();
            let mut rng = StdRng::new().unwrap();

            let &(k, v) = rng.choose(&input).unwrap();

            if let Some((m_removed, removed_value)) = m.remove(&k) {
                TestResult::from_bool(
                    m_removed.len() == m.len() - 1 && removed_value == &v
                )
            } else {
                TestResult::failed()
            }
        }
    }

    quickcheck! {
        fn check_remove_all(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let mut m: TreeMap<isize, char> = input.iter().cloned().collect();
            let mut rng = StdRng::new().unwrap();
            let mut remove_list = input.clone();
            rng.shuffle(&mut remove_list);

            for (k, _) in remove_list {
                let new_m = if let Some((m_removed, _)) = m.remove(&k) {
                    m_removed
                } else {
                    return false;
                };
                m = new_m;
                if m.contains_key(&k) {
                    return false;
                }
            }

            m.is_empty()
        }
    }

    quickcheck! {
        fn check_delete_min(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            if let Some((m_removed, (&k, _))) = m.delete_min() {
                m_removed.len() == m.len() - 1 && Some(k) == input.into_iter().min().map(|pair| pair.0)
            } else {
                true
            }
        }
    }

    quickcheck! {
        fn check_delete_max(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            if let Some((m_removed, (&k, _))) = m.delete_max() {
                m_removed.len() == m.len() - 1 && Some(k) == input.into_iter().max().map(|pair| pair.0)
            } else {
                true
            }
        }
    }

    fn match_bound<T: Ord>(x: &T, min: &Bound<T>, max: &Bound<T>) -> bool {
        let min_match = match *min {
            Bound::Unbounded => true,
            Bound::Included(ref lower) => lower <= x,
            Bound::Excluded(ref lower) => lower < x
        };

        let max_match = match *max {
            Bound::Unbounded => true,
            Bound::Included(ref upper) => x <= upper,
            Bound::Excluded(ref upper) => x < upper
        };

        min_match && max_match
    }

    quickcheck! {
        fn check_range(xs: Vec<(isize, char)>,
                       min_bound: Bound<isize>,
                       max_bound: Bound<isize>)
                -> bool
        {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            let min = match min_bound {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(ref s) => Bound::Included(s),
                Bound::Excluded(ref s) => Bound::Excluded(s),
            };

            let max = match max_bound {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(ref s) => Bound::Included(s),
                Bound::Excluded(ref s) => Bound::Excluded(s),
            };

            let res: Vec<(isize, char)> = m.range(min, max).map(|(&k, &v)| (k, v)).collect();

            for window in res.windows(2) {
                let (k0, _) = window[0];
                let (k1, _) = window[1];
                if k0 >= k1 {
                    return false;
                }
            }

            for (k, _) in input {
                let is_match = match_bound(&k, &min_bound, &max_bound);
                let is_res = res.iter().any(|pair| pair.0 == k);

                if is_match != is_res {
                    return false;
                }
            }

            true
        }
    }

    quickcheck! {
        fn check_range_rev(xs: Vec<(isize, char)>,
                           min_bound: Bound<isize>,
                           max_bound: Bound<isize>)
                -> bool
        {
            let input = filter_input(xs);
            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            let min = match min_bound {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(ref s) => Bound::Included(s),
                Bound::Excluded(ref s) => Bound::Excluded(s),
            };

            let max = match max_bound {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(ref s) => Bound::Included(s),
                Bound::Excluded(ref s) => Bound::Excluded(s),
            };

            let res: Vec<(isize, char)> = m.range(min, max).rev().map(|(&k, &v)| (k, v)).collect();

            for window in res.windows(2) {
                let (k0, _) = window[0];
                let (k1, _) = window[1];
                if k0 <= k1 {
                    return false;
                }
            }

            for (k, _) in input {
                let is_match = match_bound(&k, &min_bound, &max_bound);
                let is_res = res.iter().any(|pair| pair.0 == k);

                if is_match != is_res {
                    return false;
                }
            }

            true
        }
    }

    quickcheck! {
        fn check_eq(xs: Vec<(isize, char)>) -> bool
        {
            let mut rng = StdRng::new().unwrap();
            let input0 = filter_input(xs);
            let mut input1 = input0.clone();
            rng.shuffle(&mut input1);

            let m0: TreeMap<isize, char> = input0.into_iter().collect();
            let m1: TreeMap<isize, char> = input1.into_iter().collect();

            m0 == m1
        }
    }

    quickcheck! {
        fn check_neq(xs: Vec<(isize, char)>) -> TestResult
        {
            if xs.is_empty() {
                return TestResult::discard();
            }
            let mut rng = StdRng::new().unwrap();
            let input0 = filter_input(xs);
            let mut input1 = input0.clone();
            rng.shuffle(&mut input1);
            input1.pop();

            let m0: TreeMap<isize, char> = input0.into_iter().collect();
            let m1: TreeMap<isize, char> = input1.into_iter().collect();

            TestResult::from_bool(m0 != m1)
        }
    }

    quickcheck! {
        fn check_keys(xs: Vec<(isize, char)>) -> bool
        {
            let input = filter_input(xs);
            let mut expected: Vec<isize> = input.iter().map(|pair| pair.0).collect();

            let m: TreeMap<isize, char> = input.into_iter().collect();
            expected.sort();

            let keys: Vec<isize> = m.keys().cloned().collect();

            expected == keys
        }
    }

    quickcheck! {
        fn check_values(xs: Vec<(isize, char)>) -> bool
        {
            let input = filter_input(xs);
            let mut sorted_input = input.clone();
            sorted_input.sort();
            let expected: Vec<char> = sorted_input.into_iter().map(|pair| pair.1).collect();

            let m: TreeMap<isize, char> = input.into_iter().collect();

            let values: Vec<char> = m.values().cloned().collect();

            expected == values
        }
    }

    quickcheck! {
        fn check_insert_if_absent(xs: Vec<(isize, char)>, key: isize, value: char) -> bool
        {
            let input = filter_input(xs);

            let m: TreeMap<isize, char> = input.iter().cloned().collect();

            if input.iter().any(|&(k, _)| k == key) {
                None == m.insert_if_absent(key, value)
            } else {
                let res = m.insert_if_absent(key, value);
                res.is_some() && res.unwrap().get(&key) == Some(&value)
            }
        }
    }

    quickcheck! {
        fn check_update(xs: Vec<(char, isize)>, key: char) -> bool
        {
            let input = filter_input(xs);

            let m: TreeMap<char, isize> = input.iter().cloned().collect();

            match input.into_iter().find(|&(k, _)| k == key) {
                Some((_, value)) => {
                    let res = m.update(&key, |v| v+1);
                    res.is_some() && res.unwrap().get(&key) == Some(&(value+1))
                },
                None => m.update(&key, |v| v+1).is_none()
            }
        }
    }

    quickcheck! {
        fn check_insert_or_update(xs: Vec<(char, isize)>, key: char) -> bool
        {
            let input = filter_input(xs);

            let m: TreeMap<char, isize> = input.iter().cloned().collect();

            let m1 = m.insert_or_update(key, 1, |v| v+1);
            match input.into_iter().find(|&(k, _)| k == key) {
                Some((_, value)) => {
                    m1.get(&key) == Some(&(value+1))
                },
                None => {
                    m1.get(&key) == Some(&1)
                }
            }
        }
    }
}
