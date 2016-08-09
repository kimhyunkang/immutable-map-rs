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

#[derive(Clone, Default)]
pub struct Map<K, V> {
    root: Option<Rc<TreeNode<K, V>>>
}

pub type MapIter<'r, K, V> = tree::Iter<'r, K, V>;
pub type MapRevIter<'r, K, V> = tree::RevIter<'r, K, V>;
pub type MapRange<'r, K, V> = tree::Range<'r, K, V>;

impl<K, V> Map<K, V> where K: Ord {
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

    pub fn contains_key<Q: ?Sized + Ord>(&self, key: &Q) -> bool
        where K: Borrow<Q>
    {
        self.get(key).is_some()
    }

    pub fn range<'r, Q: Ord>(&'r self, min: Bound<&Q>, max: Bound<&Q>) -> MapRange<'r, K, V>
        where K: Borrow<Q>
    {
        tree::Range::new(&self.root, min, max)
    }
}

impl<K, V> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map { root: None }
    }

    pub fn len(&self) -> usize {
        tree::size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn iter<'r>(&'r self) -> MapIter<'r, K, V> {
        tree::Iter::new(&self.root)
    }

    pub fn rev_iter<'r>(&'r self) -> MapRevIter<'r, K, V> {
        tree::RevIter::new(&self.root)
    }
}

impl<K, V> Map<K, V> where K: Clone + Ord, V: Clone {
    pub fn insert(&self, key: K, value: V) -> Map<K, V>
    {
        let root = tree::insert(&self.root, (key, value));
        Map { root: Some(Rc::new(root)) }
    }

    pub fn delete_min(&self) -> Option<(Map<K, V>, &(K, V))>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_min(&root);
            Some((
                Map { root: new_root },
                v
            ))
        } else {
            None
        }
    }

    pub fn delete_max(&self) -> Option<(Map<K, V>, &(K, V))>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_max(&root);
            Some((
                Map { root: new_root },
                v
            ))
        } else {
            None
        }
    }

    pub fn remove<Q: ?Sized + Ord>(&self, key: &Q) -> Option<(Map<K, V>, &(K, V))>
        where K: Borrow<Q>
    {
        tree::remove(&self.root, key).map(|(new_root, v)|
            (Map { root: new_root }, v)
        )
    }
}

impl<K: Debug + Ord, V: Debug> Debug for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'r, K: Ord, V> IntoIterator for &'r Map<K, V> {
    type Item = (&'r K, &'r V);
    type IntoIter = MapIter<'r, K, V>;

    fn into_iter(self) -> MapIter<'r, K, V> {
        self.iter()
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Map<K, V>) -> bool {
        self.len() == other.len()
            && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<K: Eq, V: Eq> Eq for Map<K, V> {}

impl <K: PartialOrd, V: PartialOrd> PartialOrd for Map<K, V> {
    fn partial_cmp(&self, other: &Map<K, V>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl <K: Ord, V: Ord> Ord for Map<K, V> {
    fn cmp(&self, other: &Map<K, V>) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <'a, K: Ord, Q: ?Sized, V> Index<&'a Q> for Map<K, V>
    where K: Borrow<Q>, Q: Ord
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl <K: Ord + Clone, V: Clone> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T>(iter: T) -> Map<K, V> where T: IntoIterator<Item=(K, V)> {
        let mut m = Map::new();
        for (k, v) in iter {
            m = m.insert(k, v);
        }
        m
    }
}

#[cfg(test)]
mod test {
    use tree::balanced;

    use super::Map;
    use Bound;

    #[test]
    fn test_insert() {
        let r0 = Map::new();
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
        let r0 = Map::new();
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
        assert_eq!(&(3, 'c'), v);
    }

    #[test]
    fn test_delete_max() {
        let r0 = Map::new();
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
        assert_eq!(&(15, 'o'), v);
    }

    #[test]
    fn test_remove() {
        let r0 = Map::new();
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
        assert_eq!(&(7, 'g'), v);
    }

    #[test]
    fn test_iter() {
        let r0 = Map::new();
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
        let r0 = Map::new();
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
        let r0 = Map::new();
        let r1 = r0.insert(4, 'd');
        let r2 = r1.insert(7, 'g');

        assert!(r0.is_empty());
        assert!(!r1.is_empty());
        assert!(!r2.is_empty());
    }

    #[test]
    fn test_range() {
        let r0 = Map::new();
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
        let r0 = Map::new();
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
        let r0 = Map::new();
        let r1 = r0.insert(7, 'g');
        let r2 = r1.insert(4, 'd');

        assert_eq!("{4: 'd', 7: 'g'}", &format!("{:?}", r2));
    }
}

#[cfg(test)]
mod quickcheck {
    use map::Map;
    use Bound;

    use quickcheck::{Arbitrary, Gen, TestResult};
    use rand::{Rng, StdRng};

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

    fn filter_input<K: PartialEq, V>(input: Vec<(K, V)>) -> Vec<(K, V)> {
        let mut res: Vec<(K, V)> = Vec::new();

        for (k, v) in input.into_iter() {
            if res.iter().all(|pair| pair.0 != k) {
                res.push((k, v));
            }
        }

        res
    }

    fn from_list<K: Ord + Clone, V: Clone>(input: &[(K, V)]) -> Map<K, V> {
        let mut m = Map::new();
        for pair in input.iter() {
            m = m.insert(pair.0.clone(), pair.1.clone());
        }
        m
    }

    quickcheck! {
        fn check_length(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m = from_list(&input);

            m.len() == input.len()
        }
    }

    quickcheck! {
        fn check_is_empty(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m = from_list(&input);

            m.is_empty() == input.is_empty()
        }
    }

    quickcheck! {
        fn check_iter(xs: Vec<(isize, char)>) -> bool {
            let mut input = filter_input(xs);
            let m = from_list(&input);

            input.sort();

            let collected: Vec<(isize, char)> = m.iter().map(|(&k, &v)| (k, v)).collect();

            collected == input
        }
    }

    quickcheck! {
        fn check_iter_size_hint(xs: Vec<(isize, char)>) -> bool {
            let mut input = filter_input(xs);
            let m = from_list(&input);

            input.sort();

            let mut iter = m.iter();
            let mut expected = m.len();

            loop {
                if iter.size_hint() != (expected, Some(expected)) {
                    return false;
                }

                if iter.next().is_none() {
                    return true;
                }

                expected -= 1;
            }
        }
    }

    quickcheck! {
        fn check_rev_iter(xs: Vec<(isize, char)>) -> bool {
            let mut input = filter_input(xs);
            let m = from_list(&input);

            input.sort();
            input.reverse();

            let collected: Vec<(isize, char)> = m.rev_iter().map(|(&k, &v)| (k, v)).collect();

            collected == input
        }
    }

    quickcheck! {
        fn check_get(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m = from_list(&input);

            input.into_iter().all(|(k, v)| m.get(&k) == Some(&v))
        }
    }

    quickcheck! {
        fn check_remove(xs: Vec<(isize, char)>) -> TestResult {
            if xs.is_empty() {
                return TestResult::discard();
            }

            let input = filter_input(xs);
            let m = from_list(&input);
            let mut rng = StdRng::new().unwrap();

            let &(k, v) = rng.choose(&input).unwrap();

            if let Some((m_removed, removed_pair)) = m.remove(&k) {
                TestResult::from_bool(
                    m_removed.len() == m.len() - 1 && removed_pair.1 == v
                )
            } else {
                TestResult::failed()
            }
        }
    }

    quickcheck! {
        fn check_remove_all(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let mut m = from_list(&input);
            let mut rng = StdRng::new().unwrap();
            let mut remove_list = input.clone();
            rng.shuffle(&mut remove_list);

            for (k, _) in remove_list.into_iter() {
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
            let m = from_list(&input);

            if let Some((m_removed, &(k, _))) = m.delete_min() {
                m_removed.len() == m.len() - 1 && Some(k) == input.into_iter().min().map(|pair| pair.0)
            } else {
                true
            }
        }
    }

    quickcheck! {
        fn check_delete_max(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m = from_list(&input);

            if let Some((m_removed, &(k, _))) = m.delete_max() {
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
            let m = from_list(&input);

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

            for (k, _) in input.into_iter() {
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
            let m = from_list(&input);

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

            for (k, _) in input.into_iter() {
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

            let mut m0 = Map::new();
            for (k, v) in input0 {
                m0 = m0.insert(k, v);
            }

            let mut m1 = Map::new();
            for (k, v) in input1 {
                m1 = m1.insert(k, v);
            }

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

            let mut m0 = Map::new();
            for (k, v) in input0 {
                m0 = m0.insert(k, v);
            }

            let mut m1 = Map::new();
            for (k, v) in input1 {
                m1 = m1.insert(k, v);
            }

            TestResult::from_bool(m0 != m1)
        }
    }
}
