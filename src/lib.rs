trait BinairyTree<T: PartialOrd + PartialEq> {
    fn insert(&mut self, item: T);
    fn search(&self, item: &T) -> Option<usize>;
    fn delete(&mut self, item: &T) -> Result<(), ()>;
}

struct BST<T: PartialOrd + PartialEq> {
    head: Cell<T>,
}
#[derive(PartialEq)]
enum Cell<T: PartialOrd + PartialEq> {
    Cons {
        val: T,
        left: Box<Cell<T>>,
        right: Box<Cell<T>>,
    },
    Nil,
}
impl<T: PartialOrd + PartialEq> Cell<T> {
    fn new_with(val: T) -> Self {
        Cell::Cons {
            val,
            left: Box::new(Cell::Nil),
            right: Box::new(Cell::Nil),
        }
    }
    fn insert(&mut self, val2: T) {
        match self {
            Self::Nil => *self = Self::new_with(val2),
            Self::Cons { val, left, right } => {
                if &val2 < val {
                    left.insert(val2);
                } else {
                    right.insert(val2);
                }
            }
        }
    }
}
impl<T: PartialOrd + PartialEq> BST<T> {
    pub fn new() -> Self {
        BST { head: Cell::Nil }
    }
    pub fn new_with(val: T) -> Self {
        BST {
            head: Cell::new_with(val),
        }
    }
}
impl<T> BinairyTree<T> for BST<T>
where
    T: PartialEq + PartialOrd,
{
    fn insert(&mut self, item: T) {
        self.head.insert(item);
    }
    fn search(&self, item: &T) -> Option<usize> {
        todo!();
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
        assert!(bst.head == Cell::Nil);
        let cell = Cell::new_with(3);
        assert!(
            cell == Cell::Cons {
                val: 3,
                left: Box::new(Cell::Nil),
                right: Box::new(Cell::Nil),
            }
        );
    }
    #[test]
    fn can_insert() {
        let mut bst: BST<u32> = BST::new();
        assert!(bst.head == Cell::Nil);
        bst.insert(2);
        assert!(
            bst.head
                == Cell::Cons {
                    val: 2,
                    left: Box::new(Cell::Nil),
                    right: Box::new(Cell::Nil),
                }
        );
        bst.insert(1);
        assert!(
            bst.head
                == Cell::Cons {
                    val: 2,
                    left: Box::new(Cell::new_with(1)),
                    right: Box::new(Cell::Nil),
                }
        );
    }
}
