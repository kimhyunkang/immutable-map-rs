use std::rc::Rc;

use compare::{Compare, Natural, natural};

use tree;
use tree::TreeNode;

pub struct Set<V, C: Compare<V> = Natural<V>> {
    root: Option<Rc<TreeNode<V>>>,
    cmp: C
}

impl<V, C> Set<V, C> where C: Compare<V> {
    pub fn with_comparator(cmp: C) -> Set<V, C> {
        Set { root: None, cmp: cmp }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where C: Compare<Q, V>
    {
        fn f<'r, V, C, Q: ?Sized>(node: &'r Option<Rc<TreeNode<V>>>, cmp: &C, key: &Q)
                -> Option<&'r V> where C: Compare<Q, V>
        {
            tree::find_exact(node, |x| cmp.compare(key, x))
        }

        f(&self.root, &self.cmp, key)
    }

    pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
        where C: Compare<Q, V>
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        tree::size(&self.root)
    }
}

impl<V, C> Set<V, C> where V: Clone, C: Compare<V> + Clone {
    pub fn insert(&self, value: V) -> Set<V, C>{
        let root = tree::insert(&self.root, value, &self.cmp);
        Set { root: Some(Rc::new(root)), cmp: self.cmp.clone() }
    }
}

impl<V: Ord> Set<V> {
    pub fn new() -> Set<V> {
        Set::with_comparator(natural())
    }
}

#[cfg(test)]
mod test {
    use tree::{traverse, balanced};
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

        let mut res = Vec::new();
        traverse(&r10.root, &mut res);

        assert_eq!(expected, res);
        assert_eq!(10, r10.len());
        assert!(balanced(&r10.root));
        assert!(r10.contains(&(12, 'l')));
    }
}
