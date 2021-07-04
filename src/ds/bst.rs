//binary search tree, also known as an ordered binary tree, is a variant of
//binary trees in which the nodes are arranged in an order.
//In a binary search tree, all the nodes in the left sub-tree have a value less
//than that of the root node. Correspondingly, all the nodes in the right
//sub-tree have a value either equal to or greater than the root node.
use std::fmt::Display;

type NodeT<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    left: NodeT<T>,
    right: NodeT<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BST<T> {
    root: NodeT<T>,
}

impl<T> BST<T>
where
    T: PartialOrd,
{
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    fn _insert(node: &mut NodeT<T>, value: T) {
        let unwrapped = node.as_mut().unwrap();
        if value < unwrapped.value {
            if unwrapped.left.is_none() {
                unwrapped.left = Some(Box::new(Node::new(value)));
                return;
            }
            Self::_insert(&mut unwrapped.left, value);
        } else {
            if unwrapped.right.is_none() {
                unwrapped.right = Some(Box::new(Node::new(value)));
                return;
            }
            Self::_insert(&mut unwrapped.right, value);
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            return;
        }
        Self::_insert(&mut self.root, value)
    }

    fn _traverse_inorder(node: &NodeT<T>)
    where
        T: PartialOrd + Display,
    {
        if node.is_none() {
            return;
        }
        let unwrapped = node.as_ref().unwrap();
        Self::_traverse_inorder(&unwrapped.left);
        println!("{}", unwrapped.value);
        Self::_traverse_inorder(&unwrapped.right);
    }

    pub fn traverse_inorder(&self)
    where
        T: PartialOrd + Display,
    {
        Self::_traverse_inorder(&self.root);
    }
}
