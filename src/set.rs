use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use tree;
use tree::TreeNode;
use Bound;

#[derive(Clone, Default)]
pub struct Set<V> {
    root: Option<Rc<TreeNode<V, ()>>>,
}

impl<V: Ord> Set<V> {
    pub fn new() -> Set<V> {
        Set { root: None }
    }

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

    pub fn range<'r, Q: Ord>(&'r self, min: Bound<&Q>, max: Bound<&Q>)
            -> SetIter<tree::Range<'r, V, ()>>
        where V: Borrow<Q>
    {
        SetIter { src: tree::Range::new(&self.root, min, max) }
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
}
