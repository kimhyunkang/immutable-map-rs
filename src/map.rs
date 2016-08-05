use std::cmp::Ordering;
use std::rc::Rc;

use compare::{Compare, Natural, natural};

use tree;
use tree::TreeNode;

#[derive(Clone)]
struct KeyCompare<C> {
    key_cmp: C
}

impl<C> KeyCompare<C> {
    fn new(cmp: C) -> KeyCompare<C> {
        KeyCompare { key_cmp: cmp }
    }
}

impl<K, V, C> Compare<(K, V)> for KeyCompare<C> where C: Compare<K> {
    fn compare(&self, l: &(K, V), r: &(K, V)) -> Ordering {
        self.key_cmp.compare(&l.0, &r.0)
    }
}

pub struct Map<K, V, C: Compare<K> = Natural<K>> {
    root: Option<Rc<TreeNode<(K, V)>>>,
    cmp: KeyCompare<C>
}

impl<K, V, C> Map<K, V, C> where C: Compare<K> {
    pub fn with_comparator(cmp: C) -> Map<K, V, C> {
        Map { root: None, cmp: KeyCompare::new(cmp) }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where C: Compare<Q, K>
    {
        fn f<'r, K, V, C, Q: ?Sized>(node: &'r Option<Rc<TreeNode<(K, V)>>>, cmp: &C, key: &Q)
                -> Option<&'r V> where C: Compare<Q, K>
        {
            tree::find_exact(node, |&(ref k, _)| cmp.compare(key, k)).map(|p| &p.1)
        }

        f(&self.root, &self.cmp.key_cmp, key)
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where C: Compare<Q, K>
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        tree::size(&self.root)
    }
}

impl<K, V, C> Map<K, V, C> where K: Clone, V: Clone, C: Compare<K> + Clone {
    pub fn insert(&self, key: K, value: V) -> Map<K, V, C>
    {
        let root = tree::insert(&self.root, (key, value), &self.cmp);
        Map { root: Some(Rc::new(root)), cmp: self.cmp.clone() }
    }

    pub fn delete_min(&self) -> Option<(Map<K, V, C>, &(K, V))>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_min(&root);
            Some((
                Map { root: new_root, cmp: self.cmp.clone() },
                v
            ))
        } else {
            None
        }
    }

    pub fn delete_max(&self) -> Option<(Map<K, V, C>, &(K, V))>
    {
        if let Some(ref root) = self.root {
            let (new_root, v) = tree::delete_max(&root);
            Some((
                Map { root: new_root, cmp: self.cmp.clone() },
                v
            ))
        } else {
            None
        }
    }
}

impl<K: Ord, V> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map::with_comparator(natural())
    }
}

#[cfg(test)]
mod test {
    use tree::{traverse, balanced};
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

        let mut res = Vec::new();
        traverse(&r10.root, &mut res);

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

        let mut res = Vec::new();
        let expected = vec![(4, 'd'), (5, 'e'), (7, 'g'), (12, 'l'), (15, 'o')];
        traverse(&r7.root, &mut res);
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

        let mut res = Vec::new();
        let expected = vec![(3, 'c'), (4, 'd'), (5, 'e'), (7, 'g'), (12, 'l')];
        traverse(&r7.root, &mut res);
        assert_eq!(expected, res);
        assert_eq!(&(15, 'o'), v);
    }
}
