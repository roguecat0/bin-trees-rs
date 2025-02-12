mod simple_tree;
trait BinairyTree<T: PartialOrd + PartialEq> {
    fn insert(&mut self, item: T);
    fn search(&self, item: &T) -> &Self;
    fn delete(&mut self, item: &T) -> Result<(), ()>;
}

#[derive(PartialEq)]
enum Bst<T: PartialOrd + PartialEq> {
    Cons {
        val: T,
        left: Box<Bst<T>>,
        right: Box<Bst<T>>,
    },
    Nil,
}
impl<T: PartialOrd + PartialEq> Bst<T> {
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
impl<T> BinairyTree<T> for Bst<T>
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
        let bst: Bst<u32> = Bst::new();
        assert!(bst == Bst::Nil);
        let cell = Bst::new_with(3);
        assert!(
            cell == Bst::Cons {
                val: 3,
                left: Box::new(Bst::Nil),
                right: Box::new(Bst::Nil),
            }
        );
    }
    #[test]
    fn can_insert() {
        let mut bst: Bst<u32> = Bst::new();
        assert!(bst == Bst::Nil);
        bst.insert(2);
        assert!(
            bst == Bst::Cons {
                val: 2,
                left: Box::new(Bst::Nil),
                right: Box::new(Bst::Nil),
            }
        );
        bst.insert(1);
        assert!(
            bst == Bst::Cons {
                val: 2,
                left: Box::new(Bst::new_with(1)),
                right: Box::new(Bst::Nil),
            }
        );
    }
    #[test]
    fn can_search() {
        let mut bst = Bst::new_with(3);
        bst.insert(5);
        bst.insert(6);
        bst.insert(7);

        let node = bst.search(&8);
        assert!(node == &Bst::Nil);

        let node = bst.search(&7);
        assert!(node == &Bst::new_with(7));

        let node = bst.search(&6);
        assert!(
            node == &Bst::Cons {
                val: 6,
                left: Box::new(Bst::Nil),
                right: Box::new(Bst::new_with(7))
            }
        );
    }
}
