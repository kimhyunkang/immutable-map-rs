use std::cmp::Ordering;
use std::rc::Rc;

use compare::{Compare, Natural, natural};

static DELTA: usize = 3;
static GAMMA: usize = 2;

struct TreeNode<K, V> {
    size: usize,
    elem: (K, V),
    left: Option<Rc<TreeNode<K, V>>>,
    right: Option<Rc<TreeNode<K, V>>>
}

struct Tree<K, V, C: Compare<K> = Natural<K>> {
    root: Option<Rc<TreeNode<K, V>>>,
    cmp: C
}

impl<K, V, C> Tree<K, V, C> where C: Compare<K> {
    pub fn with_comparator(cmp: C) -> Tree<K, V, C> {
        Tree { root: None, cmp: cmp }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where C: Compare<Q, K>
    {
        fn f<'r, K, V, C, Q: ?Sized>(node: &'r Option<Rc<TreeNode<K, V>>>, cmp: &C, key: &Q)
                -> Option<&'r V> where C: Compare<Q, K>
        {
            find_exact(node, |k| cmp.compare(key, k))
        }

        f(&self.root, &self.cmp, key)
    }
}

impl<K, V, C> Tree<K, V, C> where K: Clone, V: Clone, C: Compare<K> + Clone {
    pub fn insert(&self, key: K, value: V) -> Tree<K, V, C>{
        let mut root = insert(&self.root, (key, value), &self.cmp);
        Tree { root: Some(Rc::new(root)), cmp: self.cmp.clone() }
    }
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Tree<K, V> {
        Tree::with_comparator(natural())
    }
}

impl<K, V> TreeNode<K, V> {
    fn new(elem: (K, V), left: Option<Rc<TreeNode<K, V>>>, right: Option<Rc<TreeNode<K, V>>>)
        -> TreeNode<K, V>
    {
        TreeNode {
            elem: elem,
            size: size(&left) + size(&right) + 1,
            left: left,
            right: right
        }
    }
}

fn find_exact<K, V, F>(node: &Option<Rc<TreeNode<K, V>>>, mut f: F) -> Option<&V>
    where F: FnMut(&K) -> Ordering
{
    let mut cursor = node;
    loop {
        match *cursor {
            None => return None,
            Some(ref n) => match f(&n.elem.0) {
                Ordering::Less => cursor = &n.left,
                Ordering::Equal => return Some(&n.elem.1),
                Ordering::Greater => cursor = &n.right,
            }
        }
    }
}

fn insert<K, V, C>(node: &Option<Rc<TreeNode<K, V>>>, elem: (K, V), cmp: &C) -> TreeNode<K, V>
    where K: Clone, V: Clone, C: Compare<K>
{
    match *node {
        None => TreeNode {
            size: 1,
            elem: elem,
            left: None,
            right: None
        },
        Some(ref n) => match cmp.compare(&elem.0, &n.elem.0) {
            Ordering::Less => {
                balance_right(n.elem.clone(), insert(&n.left, elem, cmp), &n.right)
            },
            Ordering::Greater => {
                balance_left(n.elem.clone(), &n.left, insert(&n.right, elem, cmp))
            },
            Ordering::Equal => TreeNode {
                size: n.size,
                elem: elem,
                left: n.left.clone(),
                right: n.right.clone()
            }
        }
    }
}

fn is_balanced(a: usize, b: usize) -> bool
{
    DELTA * (a + 1) >= b + 1
}

fn is_single(a: usize, b: usize) -> bool
{
    a + 1 < GAMMA * (b + 1)
}

fn size<K, V>(node: &Option<Rc<TreeNode<K, V>>>) -> usize {
    match *node {
        None => 0,
        Some(ref n) => n.size
    }
}

fn balance_left<K, V>(elem: (K, V),
                      left: &Option<Rc<TreeNode<K, V>>>,
                      right: TreeNode<K, V>) -> TreeNode<K, V>
    where K: Clone, V: Clone
{
    let lsize = size(left);
    if is_balanced(lsize, right.size) {
        TreeNode::new(elem, left.clone(), Some(Rc::new(right)))
    } else {
        let TreeNode { elem: r_elem, size: rsize, left: rl, right: rr } = right;
        if is_single(size(&rl), size(&rr)) {
            let new_l = TreeNode::new(elem, left.clone(), rl);
            TreeNode::new(
                r_elem,
                Some(Rc::new(new_l)),
                rr
            )
        } else {
            if let Some(ref rl_node) = rl {
                let new_l = TreeNode::new(elem, left.clone(), rl_node.left.clone());
                let new_r = TreeNode::new(r_elem, rl_node.right.clone(), rr);
                TreeNode::new(
                    rl_node.elem.clone(),
                    Some(Rc::new(new_l)),
                    Some(Rc::new(new_r))
                )
            } else {
                panic!("size invariant does not match!")
            }
        }
    }
}

fn balance_right<K, V>(elem: (K, V),
                       left: TreeNode<K, V>,
                       right: &Option<Rc<TreeNode<K, V>>>) -> TreeNode<K, V>
    where K: Clone, V: Clone
{
    let rsize = size(right);
    if is_balanced(rsize, left.size) {
        TreeNode::new(elem, Some(Rc::new(left)), right.clone())
    } else {
        let TreeNode { elem: l_elem, size: lsize, left: ll, right: lr } = left;
        if is_single(size(&lr), size(&ll)) {
            let new_r = TreeNode::new(elem, lr, right.clone());
            TreeNode::new(
                l_elem,
                ll,
                Some(Rc::new(new_r)),
            )
        } else {
            if let Some(ref lr_node) = lr {
                let new_l = TreeNode::new(l_elem, ll, lr_node.left.clone());
                let new_r = TreeNode::new(elem, lr_node.right.clone(), right.clone());
                TreeNode::new(
                    lr_node.elem.clone(),
                    Some(Rc::new(new_l)),
                    Some(Rc::new(new_r))
                )
            } else {
                panic!("size invariant does not match!")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::{Tree, TreeNode, is_balanced, size};

    fn traverse<K, V>(node: &Option<Rc<TreeNode<K, V>>>, res: &mut Vec<(K, V)>)
        where K: Clone, V: Clone
    {
        if let Some(ref n) = *node {
            traverse(&n.left, res);
            res.push(n.elem.clone());
            traverse(&n.right, res);
        }
    }

    fn balanced<K, V>(node: &Option<Rc<TreeNode<K, V>>>) -> bool
    {
        if let Some(ref n) = *node {
            is_balanced(size(&n.left), size(&n.right))
                && is_balanced(size(&n.right), size(&n.left))
                && balanced(&n.left)
                && balanced(&n.right)
        } else {
            true
        }
    }

    #[test]
    fn test_insert() {
        let r0 = Tree::new();
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
        assert!(balanced(&r10.root));
    }
}
