use std::ops::{Deref, DerefMut};

#[derive(Default, PartialEq, Debug)]
struct Bst(Option<Box<Node>>);

impl Deref for Bst {
    type Target = Option<Box<Node>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Bst {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(PartialEq, Debug)]
struct Node {
    value: i32,
    left: Bst,
    right: Bst,
}
impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: Bst::new(),
            right: Bst::new(),
        }
    }
}
impl Bst {
    fn new() -> Self {
        Self::default()
    }
    fn insert(&mut self, item: i32) {
        if self.is_none() {
            **self = Some(Box::new(Node::new(item)))
        } else if let Some(&mut Node {
            value,
            ref mut left,
            ref mut right,
        }) = self.as_mut().map(Box::as_mut)
        {
            if item < value {
                left.insert(item);
            } else if item > value {
                right.insert(item);
            }
        }
    }
    fn find_successor(&mut self) -> i32 {
        match self.as_mut().unwrap().as_mut() {
            Node {
                left: Self(Some(link)),
                ..
            } => link.left.find_successor(),
            Node {
                right: Self(Some(link)),
                ..
            } => {
                let l = link.left.find_successor();
                std::mem::replace(&mut self.as_mut().unwrap().value, l)
            }
            _ => self.take().unwrap().value,
        }
    }
    fn delete(&mut self, item: i32) {
        if let Some(node) = self.as_mut().map(Box::as_mut) {
            if item < node.value {
                node.left.delete(item);
            } else if item > node.value {
                node.right.delete(item);
            } else {
                match node {
                    Node {
                        left: Bst(None),
                        right: Bst(None),
                        ..
                    } => {
                        self.take();
                    }
                    Node {
                        left: link,
                        right: Bst(None),
                        ..
                    } => {
                        *self = Self(link.take());
                    }
                    Node {
                        right: link,
                        left: Bst(None),
                        ..
                    } => {
                        *self = Self(link.take());
                    }
                    Node {
                        right: ref mut link,
                        ..
                    } => {
                        node.value = link.find_successor();
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let bst = Bst::new();
        assert_eq!(bst, Bst(None));
    }
    #[test]
    fn can_insert() {
        let mut bst = Bst::new();
        bst.insert(3);
        assert_eq!(bst, Bst(Some(Box::new(Node::new(3)))));
        bst.insert(1);
        assert_eq!(
            bst,
            Bst(Some(Box::new(Node {
                value: 3,
                left: Bst(Some(Box::new(Node::new(1)))),
                right: Bst::default(),
            })))
        );
    }
    #[test]
    fn can_delete() {
        let mut bst = Bst::new();
        bst.insert(5);
        bst.insert(8);
        bst.insert(1);
        bst.insert(2);

        bst.delete(5);
        bst.delete(1);
        //assert_eq!(bst, Bst(Some(Box::new(Node::new(1)))));
        assert_eq!(
            bst,
            Bst(Some(Box::new(Node {
                value: 8,
                left: Bst(Some(Box::new(Node::new(2)))),
                right: Bst::default(),
            })))
        );
    }
}
