use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone, Copy)]
enum Color {
    Red,
    Black
}

struct TreeNode<K, V> {
    color: Color,
    elem: (K, V),
    left: Option<Rc<TreeNode<K, V>>>,
    right: Option<Rc<TreeNode<K, V>>>
}

struct Tree<K, V> {
    root: Option<Rc<TreeNode<K, V>>>
}

impl<K, V> Tree<K, V> {
    pub fn new() -> Tree<K, V> {
        Tree { root: None }
    }
}

impl<K, V> Tree<K, V>
    where K: Ord + Clone, V: Clone
{
    pub fn get(&self, key: &K) -> Option<&V> {
        find_exact(&self.root, key)
    }

    pub fn insert(&self, key: K, value: V) -> Tree<K, V> {
        let mut root = insert(&self.root, (key, value));
        root.color = Color::Black;
        Tree { root: Some(Rc::new(root)) }
    }
}

impl<K, V> TreeNode<K, V>
    where K: Clone, V: Clone
{
    fn change_black(&self) -> TreeNode<K, V> {
        TreeNode {
            color: Color::Black,
            elem: self.elem.clone(),
            left: self.left.clone(),
            right: self.right.clone()
        }
    }
}

fn find_exact<'a, K, V>(node: &'a Option<Rc<TreeNode<K, V>>>, key: &K) -> Option<&'a V>
    where K: Ord
{
    let mut cursor = node;
    loop {
        match *cursor {
            None => return None,
            Some(ref n) => match key.cmp(&n.elem.0) {
                Ordering::Less => cursor = &n.left,
                Ordering::Equal => return Some(&n.elem.1),
                Ordering::Greater => cursor = &n.right,
            }
        }
    }
}

fn insert<K, V>(node: &Option<Rc<TreeNode<K, V>>>, elem: (K, V)) -> TreeNode<K, V>
    where K: Ord + Clone, V: Clone
{
    match *node {
        None => TreeNode {
            color: Color::Red,
            elem: elem,
            left: None,
            right: None
        },
        Some(ref n) => match elem.0.cmp(&n.elem.0) {
            Ordering::Less => match n.color {
                Color::Black => balance_left(insert(&n.left, elem), n, &n.right),
                Color::Red => TreeNode {
                    color: Color::Red,
                    elem: n.elem.clone(),
                    left: Some(Rc::new(insert(&n.left, elem))),
                    right: n.right.clone()
                }
            },
            Ordering::Greater => match n.color {
                Color::Black => balance_right(&n.left, n, insert(&n.right, elem)),
                Color::Red => TreeNode {
                    color: Color::Red,
                    elem: n.elem.clone(),
                    left: n.left.clone(),
                    right: Some(Rc::new(insert(&n.right, elem)))
                }
            },
            Ordering::Equal => TreeNode {
                color: n.color,
                elem: elem,
                left: n.left.clone(),
                right: n.right.clone()
            }
        }
    }
}

fn balance_left<K, V>(left: TreeNode<K, V>,
                      node: &TreeNode<K, V>,
                      right: &Option<Rc<TreeNode<K, V>>>) -> TreeNode<K, V>
    where K: Ord + Clone, V: Clone
{
    if let Color::Red = left.color {
        if let Some(ref ll) = left.left {
            if let Color::Red = ll.color {
                let lr = TreeNode {
                    color: Color::Black,
                    elem: node.elem.clone(),
                    left: left.right.clone(),
                    right: right.clone()
                };

                return TreeNode {
                    color: Color::Red,
                    elem: left.elem,
                    left: Some(Rc::new(ll.change_black())),
                    right: Some(Rc::new(lr))
                }
            }
        }

        if let Some(ref lr) = left.right {
            if let Color::Red = lr.color {
                return TreeNode {
                    color: Color::Red,
                    elem: lr.elem.clone(),
                    left: Some(Rc::new(TreeNode {
                        color: Color::Black,
                        elem: left.elem,
                        left: left.left.clone(),
                        right: lr.left.clone()
                    })),
                    right: Some(Rc::new(TreeNode {
                        color: Color::Black,
                        elem: node.elem.clone(),
                        left: lr.right.clone(),
                        right: right.clone()
                    }))
                }
            }
        }
    }

    TreeNode {
        color: Color::Black,
        elem: node.elem.clone(),
        left: Some(Rc::new(left)),
        right: right.clone()
    }
}

fn balance_right<K, V>(left: &Option<Rc<TreeNode<K, V>>>,
                       node: &TreeNode<K, V>,
                       right: TreeNode<K, V>) -> TreeNode<K, V>
    where K: Ord + Clone, V: Clone
{
    if let Color::Red = right.color {
        if let Some(ref rr) = right.right {
            if let Color::Red = rr.color {
                let rl = TreeNode {
                    color: Color::Black,
                    elem: node.elem.clone(),
                    left: left.clone(),
                    right: right.left.clone()
                };

                return TreeNode {
                    color: Color::Red,
                    elem: right.elem,
                    left: Some(Rc::new(rl)),
                    right: Some(Rc::new(rr.change_black()))
                }
            }
        }

        if let Some(ref rl) = right.left {
            if let Color::Red = rl.color {
                return TreeNode {
                    color: Color::Red,
                    elem: rl.elem.clone(),
                    left: Some(Rc::new(TreeNode {
                        color: Color::Black,
                        elem: node.elem.clone(),
                        left: left.clone(),
                        right: rl.left.clone()
                    })),
                    right: Some(Rc::new(TreeNode {
                        color: Color::Black,
                        elem: right.elem,
                        left: rl.right.clone(),
                        right: right.right.clone()
                    }))
                }
            }
        }
    }

    TreeNode {
        color: Color::Black,
        elem: node.elem.clone(),
        left: left.clone(),
        right: Some(Rc::new(right))
    }
}

#[cfg(test)]
mod test {
    use std::fmt;

    use super::{Color, Tree, TreeNode};

    fn format_tree<K, V>(f: &mut fmt::Formatter, node: &TreeNode<K, V>) -> fmt::Result
        where K: fmt::Display, V: fmt::Display
    {
        try!(write!(f, "("));
        if let Some(ref l) = node.left {
            try!(format_tree(f, l));
            try!(write!(f, " "));
        }

        match node.color {
            Color::Black => try!(write!(f, "B")),
            Color::Red => try!(write!(f, "R"))
        }

        try!(write!(f, "{}{}", node.elem.0, node.elem.1));

        if let Some(ref r) = node.right {
            try!(write!(f, " "));
            try!(format_tree(f, r));
        }

        write!(f, ")")
    }

    impl<K, V> fmt::Display for Tree<K, V> where K: fmt::Display, V: fmt::Display
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.root {
                None => write!(f, "()"),
                Some(ref root) => format_tree(f, root)
            }
        }

    }

    #[test]
    fn test_insert() {
        let r0 = Tree::new();
        assert_eq!("()".to_string(), format!("{}", r0));

        let r1 = r0.insert(4, "d");
        assert_eq!("(B4d)".to_string(), format!("{}", r1));

        let r2 = r1.insert(7, "g");
        assert_eq!("(B4d (R7g))".to_string(), format!("{}", r2));

        let r3 = r2.insert(12, "l");
        assert_eq!("((B4d) B7g (B12l))".to_string(), format!("{}", r3));

        let r4 = r3.insert(15, "o");
        assert_eq!("((B4d) B7g (B12l (R15o)))".to_string(), format!("{}", r4));

        let r5 = r4.insert(3, "c");
        assert_eq!("(((R3c) B4d) B7g (B12l (R15o)))".to_string(), format!("{}", r5));

        let r6 = r5.insert(5, "e");
        assert_eq!("(((R3c) B4d (R5e)) B7g (B12l (R15o)))".to_string(), format!("{}", r6));

        let r7 = r6.insert(14, "n");
        assert_eq!("(((R3c) B4d (R5e)) B7g ((B12l) R14n (B15o)))".to_string(), format!("{}", r7));

        let r8 = r7.insert(18, "r");
        assert_eq!("(((R3c) B4d (R5e)) B7g ((B12l) R14n (B15o (R18r))))".to_string(), format!("{}", r8));

        let r9 = r8.insert(16, "p");
        assert_eq!("((((R3c) B4d (R5e)) B7g (B12l)) B14n ((B15o) B16p (B18r)))".to_string(), format!("{}", r9));

        let r10 = r9.insert(17, "q");
        assert_eq!("((((R3c) B4d (R5e)) B7g (B12l)) B14n ((B15o) B16p ((R17q) B18r)))".to_string(), format!("{}", r10));
    }
}
