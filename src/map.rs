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

    pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
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
}

impl<K: Ord, V> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map::with_comparator(natural())
    }
}
