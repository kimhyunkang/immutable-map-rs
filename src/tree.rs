use std::cmp::Ordering;
use std::rc::Rc;

use compare::{Compare, Natural, natural};

static DELTA: usize = 3;
static GAMMA: usize = 2;

struct TreeNode<V> {
    size: usize,
    elem: V,
    left: Option<Rc<TreeNode<V>>>,
    right: Option<Rc<TreeNode<V>>>
}

struct Tree<V, C: Compare<V> = Natural<V>> {
    root: Option<Rc<TreeNode<V>>>,
    cmp: C
}

impl<V, C> Tree<V, C> where C: Compare<V> {
    pub fn with_comparator(cmp: C) -> Tree<V, C> {
        Tree { root: None, cmp: cmp }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where C: Compare<Q, V>
    {
        fn f<'r, V, C, Q: ?Sized>(node: &'r Option<Rc<TreeNode<V>>>, cmp: &C, key: &Q)
                -> Option<&'r V> where C: Compare<Q, V>
        {
            find_exact(node, |x| cmp.compare(key, x))
        }

        f(&self.root, &self.cmp, key)
    }

    pub fn len(&self) -> usize {
        size(&self.root)
    }
}

impl<V, C> Tree<V, C> where V: Clone, C: Compare<V> + Clone {
    pub fn insert(&self, value: V) -> Tree<V, C>{
        let root = insert(&self.root, value, &self.cmp);
        Tree { root: Some(Rc::new(root)), cmp: self.cmp.clone() }
    }
}

impl<V: Ord> Tree<V> {
    pub fn new() -> Tree<V> {
        Tree::with_comparator(natural())
    }
}

impl<V> TreeNode<V> {
    fn new(elem: V, left: Option<Rc<TreeNode<V>>>, right: Option<Rc<TreeNode<V>>>)
        -> TreeNode<V>
    {
        TreeNode {
            elem: elem,
            size: size(&left) + size(&right) + 1,
            left: left,
            right: right
        }
    }
}

fn find_exact<V, F>(node: &Option<Rc<TreeNode<V>>>, mut f: F) -> Option<&V>
    where F: FnMut(&V) -> Ordering
{
    let mut cursor = node;
    loop {
        match *cursor {
            None => return None,
            Some(ref n) => match f(&n.elem) {
                Ordering::Less => cursor = &n.left,
                Ordering::Equal => return Some(&n.elem),
                Ordering::Greater => cursor = &n.right,
            }
        }
    }
}

fn insert<V, C>(node: &Option<Rc<TreeNode<V>>>, elem: V, cmp: &C) -> TreeNode<V>
    where V: Clone, C: Compare<V>
{
    match *node {
        None => TreeNode {
            size: 1,
            elem: elem,
            left: None,
            right: None
        },
        Some(ref n) => match cmp.compare(&elem, &n.elem) {
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

fn size<V>(node: &Option<Rc<TreeNode<V>>>) -> usize {
    match *node {
        None => 0,
        Some(ref n) => n.size
    }
}

fn balance_left<V>(elem: V,
                   left: &Option<Rc<TreeNode<V>>>,
                   right: TreeNode<V>) -> TreeNode<V>
    where V: Clone
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

fn balance_right<V>(elem: V,
                    left: TreeNode<V>,
                    right: &Option<Rc<TreeNode<V>>>) -> TreeNode<V>
    where V: Clone
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

    fn traverse<V>(node: &Option<Rc<TreeNode<V>>>, res: &mut Vec<V>)
        where V: Clone
    {
        if let Some(ref n) = *node {
            traverse(&n.left, res);
            res.push(n.elem.clone());
            traverse(&n.right, res);
        }
    }

    fn balanced<V>(node: &Option<Rc<TreeNode<V>>>) -> bool
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
    }
}
