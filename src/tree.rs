use std::borrow::Borrow;
use std::cmp::Ordering;
use std::rc::Rc;

use Bound;

static DELTA: usize = 3;
static GAMMA: usize = 2;

#[derive(Clone, Debug)]
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

pub fn insert<K, V>(node: &Option<Rc<TreeNode<K, V>>>, elem: (K, V)) -> TreeNode<K, V>
    where K: Clone + Ord, V: Clone
{
    match *node {
        None => TreeNode {
            size: 1,
            elem: elem,
            left: None,
            right: None
        },
        Some(ref n) => match elem.0.cmp(&n.elem.0) {
            Ordering::Less => {
                balance_right_move(n.elem.clone(), insert(&n.left, elem), &n.right)
            },
            Ordering::Greater => {
                balance_left_move(n.elem.clone(), &n.left, insert(&n.right, elem))
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

pub fn remove<'r, Q: ?Sized + Ord, K, V>(node: &'r Option<Rc<TreeNode<K, V>>>, key: &Q)
        -> Option<(Option<Rc<TreeNode<K, V>>>, &'r (K, V))>
    where K: Clone + Ord + Borrow<Q>, V: Clone
{
    if let Some(ref n) = *node {
        match key.cmp(n.elem.0.borrow()) {
            Ordering::Less => remove(&n.left, key).map(|(new_left, v)|
                (Some(Rc::new(balance_left(n.elem.clone(), &new_left, &n.right))), v)
            ),
            Ordering::Greater => remove(&n.right, key).map(|(new_right, v)|
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

pub struct Iter<'r, K: 'r, V: 'r> {
    stack: Vec<&'r TreeNode<K, V>>
}

impl<'r, K: 'r, V: 'r> Iter<'r, K, V> {
    pub fn new(node: &'r Option<Rc<TreeNode<K, V>>>) -> Iter<'r, K, V> {
        let mut iter = Iter { stack: Vec::new() };

        if let Some(ref n) = *node {
            iter.push_left(n);
        }

        iter
    }

    fn push_left(&mut self, node: &'r TreeNode<K, V>) {
        let mut cursor = node;

        loop {
            self.stack.push(cursor);
            match cursor.left {
                None => break,
                Some(ref l) => cursor = l
            }
        }
    }
}

impl<'r, K: 'r, V: 'r> Iterator for Iter<'r, K, V> {
    type Item = (&'r K, &'r V);

    fn next(&mut self) -> Option<(&'r K, &'r V)> {
        let top = match self.stack.pop() {
            None => return None,
            Some(t) => t
        };

        let ret = (&top.elem.0, &top.elem.1);

        if let Some(ref r) = top.right {
            self.push_left(r);
        }

        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut n = 0;
        for node in &self.stack {
            n += size(&node.right) + 1
        }
        (n, Some(n))
    }
}

pub struct RevIter<'r, K: 'r, V: 'r> {
    stack: Vec<&'r TreeNode<K, V>>
}

impl<'r, K: 'r, V: 'r> RevIter<'r, K, V> {
    pub fn new(node: &'r Option<Rc<TreeNode<K, V>>>) -> RevIter<'r, K, V> {
        let mut iter = RevIter { stack: Vec::new() };

        if let Some(ref n) = *node {
            iter.push_right(n);
        }

        iter
    }

    fn push_right(&mut self, node: &'r TreeNode<K, V>) {
        let mut cursor = node;

        loop {
            self.stack.push(cursor);
            match cursor.right {
                None => break,
                Some(ref r) => cursor = r
            }
        }
    }
}

impl<'r, K: 'r, V: 'r> Iterator for RevIter<'r, K, V> {
    type Item = (&'r K, &'r V);

    fn next(&mut self) -> Option<(&'r K, &'r V)> {
        let top = match self.stack.pop() {
            None => return None,
            Some(t) => t
        };

        let ret = (&top.elem.0, &top.elem.1);

        if let Some(ref r) = top.left {
            self.push_right(r);
        }

        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut n = 0;
        for node in &self.stack {
            n += size(&node.left) + 1
        }
        (n, Some(n))
    }
}

#[derive(Debug)]
pub struct Range<'r, K: 'r, V: 'r> {
    stack: Vec<&'r TreeNode<K, V>>,
    rev_stack: Vec<&'r TreeNode<K, V>>
}

impl<'r, K: Ord + 'r, V: 'r> Range<'r, K, V> {
    pub fn new<Q>(node: &'r Option<Rc<TreeNode<K, V>>>,
                  min: Bound<&Q>, max: Bound<&Q>)
            -> Range<'r, K, V>
        where Q: Ord, K: Borrow<Q>
    {
        let mut iter = Range { stack: Vec::new(), rev_stack: Vec::new() };

        if let Some(ref n) = *node {
            match min {
                Bound::Unbounded => iter.left_edge(n),
                Bound::Excluded(lower) => iter.left_edge_gt(n, lower),
                Bound::Included(lower) => iter.left_edge_ge(n, lower)
            }

            match max {
                Bound::Unbounded => iter.right_edge(n),
                Bound::Excluded(upper) => iter.right_edge_lt(n, upper),
                Bound::Included(upper) => iter.right_edge_le(n, upper)
            }
        }

        iter
    }

    fn left_edge(&mut self, node: &'r TreeNode<K, V>) {
        let mut cursor = node;

        loop {
            self.stack.push(cursor);
            match cursor.left {
                None => break,
                Some(ref l) => cursor = l
            }
        }
    }

    fn left_edge_gt<Q: Ord>(&mut self, node: &'r TreeNode<K, V>, key: &Q)
        where K: Borrow<Q>
    {
        let mut cursor = node;

        loop {
            if cursor.elem.0.borrow() > key {
                self.stack.push(cursor);
                match cursor.left {
                    None => break,
                    Some(ref l) => cursor = l
                }
            } else if let Some(ref r) = cursor.right {
                cursor = r;
            } else {
                break;
            }
        }
    }

    fn left_edge_ge<Q: Ord>(&mut self, node: &'r TreeNode<K, V>, key: &Q)
        where K: Borrow<Q>
    {
        let mut cursor = node;

        loop {
            match cursor.elem.0.borrow().cmp(key) {
                Ordering::Less => match cursor.right {
                    None => break,
                    Some(ref r) => cursor = r
                },
                Ordering::Equal => {
                    self.stack.push(cursor);
                    break;
                },
                Ordering::Greater => {
                    self.stack.push(cursor);
                    match cursor.left {
                        None => break,
                        Some(ref l) => cursor = l
                    }
                }
            }
        }
    }

    fn right_edge(&mut self, node: &'r TreeNode<K, V>) {
        let mut cursor = node;

        loop {
            self.rev_stack.push(cursor);
            match cursor.right {
                None => break,
                Some(ref r) => cursor = r
            }
        }
    }

    fn right_edge_lt<Q: Ord>(&mut self, node: &'r TreeNode<K, V>, key: &Q)
        where K: Borrow<Q>
    {
        let mut cursor = node;

        loop {
            if cursor.elem.0.borrow() < key {
                self.rev_stack.push(cursor);
                match cursor.right {
                    None => break,
                    Some(ref r) => cursor = r
                }
            } else if let Some(ref l) = cursor.left {
                cursor = l;
            } else {
                break;
            }
        }
    }

    fn right_edge_le<Q: Ord>(&mut self, node: &'r TreeNode<K, V>, key: &Q)
        where K: Borrow<Q>
    {
        let mut cursor = node;

        loop {
            match cursor.elem.0.borrow().cmp(key) {
                Ordering::Less => {
                    self.rev_stack.push(cursor);
                    match cursor.right {
                        None => break,
                        Some(ref r) => cursor = r
                    }
                },
                Ordering::Equal => {
                    self.rev_stack.push(cursor);
                    break;
                },
                Ordering::Greater => match cursor.left {
                    None => break,
                    Some(ref l) => cursor = l
                }
            }
        }
    }
}

impl<'r, K: Ord + 'r, V: 'r> Iterator for Range<'r, K, V> {
    type Item = (&'r K, &'r V);

    fn next(&mut self) -> Option<(&'r K, &'r V)> {
        let top = match self.stack.pop() {
            None => return None,
            Some(t) => t
        };

        let ret = (&top.elem.0, &top.elem.1);

        if let Some(rev_top) = self.rev_stack.last() {
            if rev_top.elem.0 < top.elem.0 {
                return None;
            }
        } else {
            return None;
        }

        if let Some(ref r) = top.right {
            self.left_edge(r);
        }

        Some(ret)
    }
}

impl<'r, K: Ord + 'r, V: 'r> DoubleEndedIterator for Range<'r, K, V> {
    fn next_back(&mut self) -> Option<(&'r K, &'r V)> {
        let top = match self.rev_stack.pop() {
            None => return None,
            Some(t) => t
        };

        let ret = (&top.elem.0, &top.elem.1);

        if let Some(rev_top) = self.stack.last() {
            if top.elem.0 < rev_top.elem.0 {
                return None;
            }
        } else {
            return None;
        }

        if let Some(ref r) = top.left {
            self.right_edge(r);
        }

        Some(ret)
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
