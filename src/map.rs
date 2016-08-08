use std::borrow::Borrow;
use std::rc::Rc;

use tree;
use tree::TreeNode;

pub struct Map<K, V> {
    root: Option<Rc<TreeNode<K, V>>>
}

pub type MapIter<'r, K, V> = tree::Iter<'r, K, V>;
pub type MapRevIter<'r, K, V> = tree::RevIter<'r, K, V>;

impl<K, V> Map<K, V> where K: Ord {
    pub fn new() -> Map<K, V> {
        Map { root: None }
    }

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

#[cfg(test)]
mod test {
    use tree::balanced;
    use super::Map;

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

        let res: Vec<_> = r10.iter().cloned().collect();

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
        let res: Vec<_> = r7.iter().cloned().collect();

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
        let res: Vec<_> = r7.iter().cloned().collect();

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
        let res: Vec<_> = r7.iter().cloned().collect();

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

        let res: Vec<(usize, char)> = r10.iter().cloned().collect();

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

        let res: Vec<(usize, char)> = r10.rev_iter().cloned().collect();

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
}

#[cfg(test)]
mod quickcheck {
    use super::Map;

    use rand::{Rng, StdRng};

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

            let collected: Vec<(isize, char)> = m.iter().cloned().collect();

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

            let collected: Vec<(isize, char)> = m.rev_iter().cloned().collect();

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
        fn check_remove(xs: Vec<(isize, char)>) -> bool {
            let input = filter_input(xs);
            let m = from_list(&input);
            let mut rng = StdRng::new().unwrap();

            if let Some(&(k, v)) = rng.choose(&input) {
                if let Some((m_removed, removed_pair)) = m.remove(&k) {
                    m_removed.len() == m.len() - 1 && removed_pair.1 == v
                } else {
                    false
                }
            } else {
                true
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
}
