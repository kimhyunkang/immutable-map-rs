use std::borrow::Borrow;
use std::rc::Rc;

use tree;
use tree::TreeNode;

pub struct Map<K, V> {
    root: Option<Rc<TreeNode<K, V>>>
}

pub type MapIter<'r, K, V> = tree::Iter<'r, K, V>;

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

    pub fn iter<'r>(&'r self) -> MapIter<'r, K, V> {
        tree::Iter::new(&self.root)
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

        let mut res = Vec::new();
        let expected = vec![(3, 'c'), (4, 'd'), (5, 'e'), (12, 'l'), (15, 'o')];
        traverse(&r7.root, &mut res);
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
}
