trait BinairyTree<T: PartialOrd + PartialEq> {
    fn insert(&mut self, item: T);
    fn search(&self, item: &T) -> &Self;
    fn delete(&mut self, item: &T) -> Result<(), ()>;
}

#[derive(PartialEq)]
enum BST<T: PartialOrd + PartialEq> {
    Cons {
        val: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Nil,
}
impl<T: PartialOrd + PartialEq> BST<T> {
    fn new_with(val: T) -> Self {
        Self::Cons {
            val,
            left: Box::new(Self::Nil),
            right: Box::new(Self::Nil),
        }
    }
    fn new() -> Self {
        Self::Nil
    }
}
impl<T> BinairyTree<T> for BST<T>
where
    T: PartialEq + PartialOrd,
{
    fn insert(&mut self, item: T) {
        match self {
            Self::Nil => *self = Self::new_with(item),
            Self::Cons { val, left, right } => {
                if &item < val {
                    left.insert(item);
                } else if &item > val {
                    right.insert(item);
                }
            }
        }
    }
    fn search(&self, item: &T) -> &Self {
        match self {
            Self::Nil => self,
            Self::Cons { val, .. } if val == item => self,
            Self::Cons { val, left, right } => {
                if item < val {
                    left.search(item)
                } else {
                    right.search(item)
                }
            }
        }
    }
    fn delete(&mut self, item: &T) -> Result<(), ()> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let bst: BST<u32> = BST::new();
        assert!(bst == BST::Nil);
        let cell = BST::new_with(3);
        assert!(
            cell == BST::Cons {
                val: 3,
                left: Box::new(BST::Nil),
                right: Box::new(BST::Nil),
            }
        );
    }
    #[test]
    fn can_insert() {
        let mut bst: BST<u32> = BST::new();
        assert!(bst == BST::Nil);
        bst.insert(2);
        assert!(
            bst == BST::Cons {
                val: 2,
                left: Box::new(BST::Nil),
                right: Box::new(BST::Nil),
            }
        );
        bst.insert(1);
        assert!(
            bst == BST::Cons {
                val: 2,
                left: Box::new(BST::new_with(1)),
                right: Box::new(BST::Nil),
            }
        );
    }
    #[test]
    fn can_search() {
        let mut bst = BST::new_with(3);
        bst.insert(5);
        bst.insert(6);
        bst.insert(7);

        let node = bst.search(&8);
        assert!(node == &BST::Nil);

        let node = bst.search(&7);
        assert!(node == &BST::new_with(7));

        let node = bst.search(&6);
        assert!(
            node == &BST::Cons {
                val: 6,
                left: Box::new(BST::Nil),
                right: Box::new(BST::new_with(7))
            }
        );
    }
}
