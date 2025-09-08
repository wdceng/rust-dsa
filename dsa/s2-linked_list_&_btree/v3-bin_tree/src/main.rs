use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                } else {
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(Node {
                    data,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }))
            }
        }
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_left_first(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_left_first(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{}{:?}", spc, bd.data);
            bd.right.print_left_first(dp + 1);
        }
    }
}

fn main() {
    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(1);
    t.add_sorted(94);
    t.add_sorted(54);
    t.add_sorted(3);
    t.print_left_first(0);

    // println!("t = {:?}", t);
}
