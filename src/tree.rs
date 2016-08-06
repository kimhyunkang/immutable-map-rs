use std::cmp::Ordering;
use std::rc::Rc;

use compare::Compare;

static DELTA: usize = 3;
static GAMMA: usize = 2;

#[derive(Clone)]
pub struct TreeNode<K, V> {
    size: usize,
    elem: (K, V),
    left: Option<Rc<TreeNode<K, V>>>,
    right: Option<Rc<TreeNode<K, V>>>
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

pub fn find_exact<K, V, F>(node: &Option<Rc<TreeNode<K, V>>>, mut f: F) -> Option<&(K, V)>
    where F: FnMut(&K) -> Ordering
{
    let mut cursor = node;
    loop {
        match *cursor {
            None => return None,
            Some(ref n) => match f(&n.elem.0) {
                Ordering::Less => cursor = &n.left,
                Ordering::Equal => return Some(&n.elem),
                Ordering::Greater => cursor = &n.right,
            }
        }
    }
}

pub fn delete_min<K, V>(node: &TreeNode<K, V>) -> (Option<Rc<TreeNode<K, V>>>, &(K, V))
    where K: Clone, V: Clone
{
    match node.left {
        None => (node.right.clone(), &node.elem),
        Some(ref l) => {
            let (new_left, v) = delete_min(l);
            let new_node = balance_left(node.elem.clone(), &new_left, &node.right);
            (Some(Rc::new(new_node)), v)
        }
    }
}

pub fn delete_max<K, V>(node: &TreeNode<K, V>) -> (Option<Rc<TreeNode<K, V>>>, &(K, V))
    where K: Clone, V: Clone
{
    match node.right {
        None => (node.left.clone(), &node.elem),
        Some(ref r) => {
            let (new_right, v) = delete_max(r);
            let new_node = balance_right(node.elem.clone(), &node.left, &new_right);
            (Some(Rc::new(new_node)), v)
        }
    }
}

pub fn insert<K, V, C>(node: &Option<Rc<TreeNode<K, V>>>, elem: (K, V), cmp: &C) -> TreeNode<K, V>
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
                balance_right_move(n.elem.clone(), insert(&n.left, elem, cmp), &n.right)
            },
            Ordering::Greater => {
                balance_left_move(n.elem.clone(), &n.left, insert(&n.right, elem, cmp))
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

pub fn remove<'r, Q: ?Sized, K, V, C>(node: &'r Option<Rc<TreeNode<K, V>>>, key: &Q, cmp: &C)
        -> Option<(Option<Rc<TreeNode<K, V>>>, &'r (K, V))>
    where K: Clone, V: Clone, C: Compare<Q, K>
{
    if let Some(ref n) = *node {
        match cmp.compare(key, &n.elem.0) {
            Ordering::Less => remove(&n.left, key, cmp).map(|(new_left, v)|
                (Some(Rc::new(balance_left(n.elem.clone(), &new_left, &n.right))), v)
            ),
            Ordering::Greater => remove(&n.right, key, cmp).map(|(new_right, v)|
                (Some(Rc::new(balance_right(n.elem.clone(), &n.left, &new_right))), v)
            ),
            Ordering::Equal => Some((glue(&n.left, &n.right), &n.elem))
        }
    } else {
        None
    }
}

// merge the two trees together.
// assumes that left.rightmost < right.leftmost
fn glue<K, V>(left: &Option<Rc<TreeNode<K, V>>>, right: &Option<Rc<TreeNode<K, V>>>)
        -> Option<Rc<TreeNode<K, V>>>
    where K: Clone, V: Clone
{
    match *left {
        None => right.clone(),
        Some(ref l) => match *right {
            None => left.clone(),
            Some(ref r) =>
                if l.size > r.size {
                    let (new_l, elem) = delete_max(l);
                    Some(Rc::new(balance_left_move(elem.clone(), &new_l, (**r).clone())))
                } else {
                    let (new_r, elem) = delete_min(r);
                    Some(Rc::new(balance_right_move(elem.clone(), (**l).clone(), &new_r)))
                }
        }
    }
}

pub fn is_balanced(a: usize, b: usize) -> bool
{
    DELTA * (a + 1) >= b + 1
}

fn is_single(a: usize, b: usize) -> bool
{
    a + 1 < GAMMA * (b + 1)
}

pub fn size<K, V>(node: &Option<Rc<TreeNode<K, V>>>) -> usize {
    match *node {
        None => 0,
        Some(ref n) => n.size
    }
}

fn balance_left<K, V>(elem: (K, V),
                      left: &Option<Rc<TreeNode<K, V>>>,
                      right: &Option<Rc<TreeNode<K, V>>>) -> TreeNode<K, V>
    where K: Clone, V: Clone
{
    if let Some(ref r) = *right {
        balance_left_move(elem, left, (**r).clone())
    } else {
        TreeNode::new(elem, left.clone(), None)
    }
}

fn balance_left_move<K, V>(elem: (K, V),
                           left: &Option<Rc<TreeNode<K, V>>>,
                           right: TreeNode<K, V>) -> TreeNode<K, V>
    where K: Clone, V: Clone
{
    let lsize = size(left);
    if is_balanced(lsize, right.size) {
        TreeNode::new(elem, left.clone(), Some(Rc::new(right)))
    } else {
        let TreeNode { elem: r_elem, size: _, left: rl, right: rr } = right;
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
                       left: &Option<Rc<TreeNode<K, V>>>,
                       right: &Option<Rc<TreeNode<K, V>>>) -> TreeNode<K, V>
    where K: Clone, V: Clone
{
    if let Some(ref l) = *left {
        balance_right_move(elem, (**l).clone(), right)
    } else {
        TreeNode::new(elem, None, right.clone())
    }
}

fn balance_right_move<K, V>(elem: (K, V),
                            left: TreeNode<K, V>,
                            right: &Option<Rc<TreeNode<K, V>>>) -> TreeNode<K, V>
    where K: Clone, V: Clone
{
    let rsize = size(right);
    if is_balanced(rsize, left.size) {
        TreeNode::new(elem, Some(Rc::new(left)), right.clone())
    } else {
        let TreeNode { elem: l_elem, size: _, left: ll, right: lr } = left;
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
pub fn traverse<K, V>(node: &Option<Rc<TreeNode<K, V>>>, res: &mut Vec<(K, V)>)
    where K: Clone, V: Clone
{
    if let Some(ref n) = *node {
        traverse(&n.left, res);
        res.push(n.elem.clone());
        traverse(&n.right, res);
    }
}

#[cfg(test)]
pub fn balanced<K, V>(node: &Option<Rc<TreeNode<K, V>>>) -> bool
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
