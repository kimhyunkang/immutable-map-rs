use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::iter::{FromIterator, Peekable};
use std::rc::Rc;

use tree;
use tree::TreeNode;
use Bound;

#[derive(Clone, Default)]
pub struct Set<V> {
    root: Option<Rc<TreeNode<V, ()>>>,
}

impl<V: Ord> Set<V> {
    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
        where V: Borrow<Q>
    {
        fn f<'r, V: Borrow<Q>, Q: Ord + ?Sized>(node: &'r Option<Rc<TreeNode<V, ()>>>, key: &Q)
                -> Option<&'r V>
        {
            tree::find_exact(node, |x| key.cmp(x.borrow())).map(|p| &p.0)
        }

        f(&self.root, key)
    }

    pub fn contains<Q: Ord + ?Sized>(&self, key: &Q) -> bool
        where V: Borrow<Q>
    {
        self.get(key).is_some()
    }

    pub fn range<'r, Q: Ord>(&'r self, min: Bound<&Q>, max: Bound<&Q>)
            -> SetIter<tree::Range<'r, V, ()>>
        where V: Borrow<Q>
    {
        SetIter { src: tree::Range::new(&self.root, min, max) }
    }

    pub fn intersection<'r>(&'r self, other: &'r Set<V>) -> Intersection<'r, V> {
        Intersection {
            a: tree::Iter::new(&self.root).peekable(),
            b: tree::Iter::new(&other.root).peekable()
        }
    }

    pub fn union<'r>(&'r self, other: &'r Set<V>) -> Union<'r, V> {
        Union {
            a: tree::Iter::new(&self.root).peekable(),
            b: tree::Iter::new(&other.root).peekable()
        }
    }

    pub fn difference<'r>(&'r self, other: &'r Set<V>) -> Difference<'r, V> {
        Difference {
            a: tree::Iter::new(&self.root).peekable(),
            b: tree::Iter::new(&other.root).peekable()
        }
    }
}

impl<V> Set<V> {
    pub fn new() -> Set<V> {
        Set { root: None }
    }

    pub fn len(&self) -> usize {
        tree::size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn iter<'r>(&'r self) -> SetIter<tree::Iter<'r, V, ()>> {
        SetIter { src: tree::Iter::new(&self.root) }
    }

    pub fn rev_iter<'r>(&'r self) -> SetIter<tree::RevIter<'r, V, ()>> {
        SetIter { src: tree::RevIter::new(&self.root) }
    }
}

impl<V: Ord> Set<V> where V: Clone {
    pub fn insert(&self, value: V) -> Set<V>
    {
        let root = tree::insert(&self.root, (value, ()));
        Set { root: Some(Rc::new(root)) }
    }

    pub fn delete_min(&self) -> Option<(Set<V>, &V)>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_min(&root);
            Some((
                Set { root: new_root },
                &v.0
            ))
        } else {
            None
        }
    }

    pub fn delete_max(&self) -> Option<(Set<V>, &V)>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_max(&root);
            Some((
                Set { root: new_root },
                &v.0
            ))
        } else {
            None
        }
    }

    pub fn remove<Q: Ord + ?Sized>(&self, key: &Q) -> Option<(Set<V>, &V)>
        where V: Borrow<Q>
    {
        tree::remove(&self.root, key).map(|(new_root, v)|
            (Set { root: new_root }, &v.0)
        )
    }
}

impl<V: Debug + Ord> Debug for Set<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

pub struct SetIter<I> {
    src: I
}

impl<'r, I: 'r, V: 'r> Iterator for SetIter<I> where I: Iterator<Item=(&'r V, &'r ())> {
    type Item = &'r V;

    fn next(&mut self) -> Option<&'r V> {
        self.src.next().map(|p| p.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.src.size_hint()
    }
}

impl<'r, I: 'r, V: 'r> DoubleEndedIterator for SetIter<I>
    where I: DoubleEndedIterator<Item=(&'r V, &'r ())>
{
    fn next_back(&mut self) -> Option<&'r V> {
        self.src.next_back().map(|p| p.0)
    }
}

impl<'r, V: Ord> IntoIterator for &'r Set<V> {
    type Item = &'r V;
    type IntoIter = SetIter<tree::Iter<'r, V, ()>>;

    fn into_iter(self) -> SetIter<tree::Iter<'r, V, ()>> {
        self.iter()
    }
}

impl <V: PartialEq> PartialEq for Set<V> {
    fn eq(&self, other: &Set<V>) -> bool {
        self.len() == other.len()
            && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl <V: Eq> Eq for Set<V> {}

impl <V: PartialOrd> PartialOrd for Set<V> {
    fn partial_cmp(&self, other: &Set<V>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl <V: Ord> Ord for Set<V> {
    fn cmp(&self, other: &Set<V>) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <V: Ord + Clone> FromIterator<V> for Set<V> {
    fn from_iter<T>(iter: T) -> Set<V> where T: IntoIterator<Item=V> {
        let mut s = Set::new();
        for v in iter {
            s = s.insert(v);
        }
        s
    }
}

pub struct Intersection<'r, V: 'r> {
    a: Peekable<tree::Iter<'r, V, ()>>,
    b: Peekable<tree::Iter<'r, V, ()>>
}

impl<'r, V: Ord + 'r> Iterator for Intersection<'r, V> {
    type Item = &'r V;

    fn next(&mut self) -> Option<&'r V> {
        loop {
            let cmp = match (self.a.peek(), self.b.peek()) {
                (None, _) => return None,
                (_, None) => return None,
                (Some(a), Some(b)) => a.cmp(b)
            };

            match cmp {
                Ordering::Less => {
                    self.a.next();
                },
                Ordering::Equal => {
                    self.b.next();
                    return self.a.next().map(|pair| pair.0);
                },
                Ordering::Greater => {
                    self.b.next();
                }
            }
        }
    }
}

pub struct Union<'r, V: 'r> {
    a: Peekable<tree::Iter<'r, V, ()>>,
    b: Peekable<tree::Iter<'r, V, ()>>
}

impl <'r, V: Ord + 'r> Iterator for Union<'r, V> {
    type Item = &'r V;

    fn next(&mut self) -> Option<&'r V> {
        loop {
            let cmp = match (self.a.peek(), self.b.peek()) {
                (_, None) => Ordering::Less,
                (None, _) => Ordering::Greater,
                (Some(a), Some(b)) => a.cmp(b)
            };

            match cmp {
                Ordering::Less => {
                    return self.a.next().map(|pair| pair.0);
                },
                Ordering::Equal => {
                    self.b.next();
                    return self.a.next().map(|pair| pair.0);
                },
                Ordering::Greater => {
                    return self.b.next().map(|pair| pair.0);
                }
            }
        }
    }
}

pub struct Difference<'r, V: 'r> {
    a: Peekable<tree::Iter<'r, V, ()>>,
    b: Peekable<tree::Iter<'r, V, ()>>
}

impl<'r, V: Ord + 'r> Iterator for Difference<'r, V> {
    type Item = &'r V;

    fn next(&mut self) -> Option<&'r V> {
        loop {
            let cmp = match (self.a.peek(), self.b.peek()) {
                (_, None) => Ordering::Less,
                (None, _) => return None,
                (Some(a), Some(b)) => a.cmp(b)
            };

            match cmp {
                Ordering::Less => {
                    return self.a.next().map(|pair| pair.0);
                },
                Ordering::Equal => {
                    self.a.next();
                    self.b.next();
                },
                Ordering::Greater => {
                    self.b.next();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use tree::balanced;
    use Bound;

    use super::Set;

    #[test]
    fn test_insert() {
        let r0 = Set::new();
        let r1 = r0.insert((4, 'd'));
        let r2 = r1.insert((7, 'g'));
        let r3 = r2.insert((12, 'l'));
        let r4 = r3.insert((15, 'o'));
        let r5 = r4.insert((3, 'c'));
        let r6 = r5.insert((5, 'e'));
        let r7 = r6.insert((14, 'n'));
        let r8 = r7.insert((18, 'r'));
        let r9 = r8.insert((16, 'p'));
        let r10 = r9.insert((17, 'q'));

        let expected = vec![
            (3, 'c'), (4, 'd'), (5, 'e'), (7, 'g'),
            (12, 'l'), (14, 'n'), (15, 'o'), (16, 'p'),
            (17, 'q'), (18, 'r')
        ];

        let res: Vec<(usize, char)> = r10.iter().cloned().collect();

        assert_eq!(expected, res);
        assert_eq!(10, r10.len());
        assert!(balanced(&r10.root));
        assert!(r10.contains(&(12, 'l')));
    }

    #[test]
    fn test_delete_min() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let (r7, v) = r6.delete_min().unwrap();

        let expected = vec![4, 5, 7, 12, 15];

        let res: Vec<usize> = r7.iter().cloned().collect();

        assert_eq!(expected, res);
        assert_eq!(&3, v);
    }

    #[test]
    fn test_delete_max() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let (r7, v) = r6.delete_max().unwrap();

        let expected = vec![3, 4, 5, 7, 12];
        let res: Vec<usize> = r7.iter().cloned().collect();

        assert_eq!(expected, res);
        assert_eq!(&15, v);
    }

    #[test]
    fn test_remove() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let (r7, v) = r6.remove(&7).unwrap();

        let expected = vec![3, 4, 5, 12, 15];
        let res: Vec<usize> = r7.iter().cloned().collect();

        assert_eq!(expected, res);
        assert_eq!(&7, v);
    }

    #[test]
    fn test_iter() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let r7 = r6.insert(14);
        let r8 = r7.insert(18);
        let r9 = r8.insert(16);
        let r10 = r9.insert(17);

        let expected = vec![3, 4, 5, 7, 12, 14, 15, 16, 17, 18];
        let res: Vec<usize> = r10.iter().cloned().collect();

        assert_eq!(expected, res);

        assert_eq!((10, Some(10)), r10.iter().size_hint());
    }

    #[test]
    fn test_rev_iter() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let r7 = r6.insert(14);
        let r8 = r7.insert(18);
        let r9 = r8.insert(16);
        let r10 = r9.insert(17);

        let expected = vec![18, 17, 16, 15, 14, 12, 7, 5, 4, 3];
        let res: Vec<usize> = r10.rev_iter().cloned().collect();

        assert_eq!(expected, res);

        assert_eq!((10, Some(10)), r10.rev_iter().size_hint());
    }

    #[test]
    fn test_is_empty() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);

        assert!(r0.is_empty());
        assert!(!r1.is_empty());
        assert!(!r2.is_empty());
    }

    #[test]
    fn test_range() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let r7 = r6.insert(14);
        let r8 = r7.insert(18);
        let r9 = r8.insert(16);
        let r10 = r9.insert(17);

        let expected = vec![7, 12, 14, 15, 16];

        let res: Vec<usize> = r10.range(Bound::Included(&6), Bound::Excluded(&17))
                                 .cloned().collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_range_rev() {
        let r0 = Set::new();
        let r1 = r0.insert(4);
        let r2 = r1.insert(7);
        let r3 = r2.insert(12);
        let r4 = r3.insert(15);
        let r5 = r4.insert(3);
        let r6 = r5.insert(5);
        let r7 = r6.insert(14);
        let r8 = r7.insert(18);
        let r9 = r8.insert(16);
        let r10 = r9.insert(17);

        let expected = vec![16, 15, 14, 12, 7];

        let res: Vec<usize> = r10.range(Bound::Included(&6), Bound::Excluded(&17))
                                 .rev()
                                 .cloned().collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_debug() {
        let r0 = Set::new();
        let r1 = r0.insert(7);
        let r2 = r1.insert(4);

        assert_eq!("{4, 7}", &format!("{:?}", r2));
    }

    #[test]
    fn test_eq() {
        let a = Set::new().insert(3).insert(1).insert(2);
        let b = Set::new().insert(2).insert(3).insert(1).insert(2);

        assert_eq!(a, b);
    }

    #[test]
    fn test_neq() {
        let a = Set::new().insert(3).insert(1).insert(2);
        let b = Set::new().insert(2).insert(4).insert(1);

        assert!(a != b);
    }
}

#[cfg(test)]
mod quickcheck {
    use set::Set;
    use Bound;

    use quickcheck::TestResult;
    use rand::{Rng, StdRng};

    fn filter_input<V: PartialEq>(input: Vec<V>) -> Vec<V> {
        let mut res: Vec<V> = Vec::new();

        for v in input {
            if res.iter().all(|x| x != &v) {
                res.push(v);
            }
        }

        res
    }

    quickcheck! {
        fn check_length(xs: Vec<isize>) -> bool {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            m.len() == input.len()
        }
    }

    quickcheck! {
        fn check_is_empty(xs: Vec<isize>) -> bool {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            m.is_empty() == input.is_empty()
        }
    }

    quickcheck! {
        fn check_iter(xs: Vec<isize>) -> bool {
            let mut input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            input.sort();

            let collected: Vec<isize> = m.iter().cloned().collect();

            collected == input
        }
    }

    quickcheck! {
        fn check_iter_size_hint(xs: Vec<isize>) -> bool {
            let mut input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

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
        fn check_rev_iter(xs: Vec<isize>) -> bool {
            let mut input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            input.sort();
            input.reverse();

            let collected: Vec<isize> = m.rev_iter().cloned().collect();

            collected == input
        }
    }

    quickcheck! {
        fn check_contains(xs: Vec<isize>) -> bool {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            input.into_iter().all(|v| m.contains(&v))
        }
    }

    quickcheck! {
        fn check_remove(xs: Vec<isize>) -> TestResult {
            if xs.is_empty() {
                return TestResult::discard();
            }

            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();
            let mut rng = StdRng::new().unwrap();

            let &v = rng.choose(&input).unwrap();

            if let Some((m_removed, &removed)) = m.remove(&v) {
                TestResult::from_bool(
                    m_removed.len() == m.len() - 1 && removed == v
                )
            } else {
                TestResult::failed()
            }
        }
    }

    quickcheck! {
        fn check_remove_all(xs: Vec<isize>) -> bool {
            let input = filter_input(xs);
            let mut m: Set<isize> = input.iter().cloned().collect();
            let mut rng = StdRng::new().unwrap();
            let mut remove_list = input.clone();
            rng.shuffle(&mut remove_list);

            for v in remove_list {
                let new_m = if let Some((m_removed, _)) = m.remove(&v) {
                    m_removed
                } else {
                    return false;
                };
                m = new_m;
                if m.contains(&v) {
                    return false;
                }
            }

            m.is_empty()
        }
    }

    quickcheck! {
        fn check_delete_min(xs: Vec<isize>) -> bool {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            if let Some((m_removed, &v)) = m.delete_min() {
                m_removed.len() == m.len() - 1 && Some(v) == input.into_iter().min()
            } else {
                true
            }
        }
    }

    quickcheck! {
        fn check_delete_max(xs: Vec<isize>) -> bool {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

            if let Some((m_removed, &v)) = m.delete_max() {
                m_removed.len() == m.len() - 1 && Some(v) == input.into_iter().max()
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
        fn check_range(xs: Vec<isize>, min_bound: Bound<isize>, max_bound: Bound<isize>) -> bool
        {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

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

            let res: Vec<isize> = m.range(min, max).cloned().collect();

            for window in res.windows(2) {
                if window[0] >= window[1] {
                    return false;
                }
            }

            for v in input {
                let is_match = match_bound(&v, &min_bound, &max_bound);
                let is_res = res.contains(&v);

                if is_match != is_res {
                    return false;
                }
            }

            true
        }
    }

    quickcheck! {
        fn check_range_rev(xs: Vec<isize>, min_bound: Bound<isize>, max_bound: Bound<isize>)
                -> bool
        {
            let input = filter_input(xs);
            let m: Set<isize> = input.iter().cloned().collect();

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

            let res: Vec<isize> = m.range(min, max).rev().cloned().collect();

            for window in res.windows(2) {
                if window[0] <= window[1] {
                    return false;
                }
            }

            for v in input {
                let is_match = match_bound(&v, &min_bound, &max_bound);
                let is_res = res.contains(&v);

                if is_match != is_res {
                    return false;
                }
            }

            true
        }
    }

    quickcheck! {
        fn check_eq(xs: Vec<isize>) -> bool
        {
            let mut rng = StdRng::new().unwrap();
            let input0 = filter_input(xs);
            let mut input1 = input0.clone();
            rng.shuffle(&mut input1);

            let m0: Set<isize> = input0.into_iter().collect();
            let m1: Set<isize> = input1.into_iter().collect();

            m0 == m1
        }
    }

    quickcheck! {
        fn check_neq(xs: Vec<isize>) -> TestResult
        {
            if xs.is_empty() {
                return TestResult::discard();
            }
            let mut rng = StdRng::new().unwrap();
            let input0 = filter_input(xs);
            let mut input1 = input0.clone();
            rng.shuffle(&mut input1);
            input1.pop();

            let m0: Set<isize> = input0.into_iter().collect();
            let m1: Set<isize> = input1.into_iter().collect();

            TestResult::from_bool(m0 != m1)
        }
    }

    quickcheck! {
        fn check_intersection(input0: Vec<isize>, input1: Vec<isize>) -> bool {
            let xs = filter_input(input0);
            let ys = filter_input(input1);

            let mut intersection = Vec::new();
            for x in &xs {
                if ys.contains(x) {
                    intersection.push(*x);
                }
            }

            intersection.sort();

            let x_set: Set<isize> = xs.into_iter().collect();
            let y_set: Set<isize> = ys.into_iter().collect();

            let res: Vec<isize> = x_set.intersection(&y_set).cloned().collect();

            res == intersection
        }
    }

    quickcheck! {
        fn check_union(input0: Vec<isize>, input1: Vec<isize>) -> bool {
            let xs = filter_input(input0);
            let ys = filter_input(input1);

            let mut union = xs.clone();
            for y in &ys {
                if !union.contains(y) {
                    union.push(*y);
                }
            }

            union.sort();

            let x_set: Set<isize> = xs.into_iter().collect();
            let y_set: Set<isize> = ys.into_iter().collect();

            let res: Vec<isize> = x_set.union(&y_set).cloned().collect();

            res == union
        }
    }

    quickcheck! {
        fn check_difference(input0: Vec<isize>, input1: Vec<isize>) -> bool {
            let xs = filter_input(input0);
            let ys = filter_input(input1);

            let mut difference = Vec::new();
            for x in &xs {
                if !ys.contains(x) {
                    difference.push(*x);
                }
            }

            difference.sort();

            let x_set: Set<isize> = xs.into_iter().collect();
            let y_set: Set<isize> = ys.into_iter().collect();

            let res: Vec<isize> = x_set.difference(&y_set).cloned().collect();

            res == difference
        }
    }
}
