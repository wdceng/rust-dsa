use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    h: i8, // Could be boolean for Red and Black, but math is easier with small int.
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> Node<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // Result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No right node how can we rotate?
        };

        // Move left of right node to right of start node
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        // Set the results left node to the start node
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }

    pub fn rot_right(mut self) -> Box<Self> {
        // Result is the right node
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No right node how can we rotate?
        };

        // Move left of right node to right of start node
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        // Set the results left node to the start node
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }
    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(Node {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };

        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }

        self.set_height();
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
            println!("{}:{}{:?}", bd.h, spc, bd.data);
            bd.right.print_left_first(dp + 1);
        }
    }
}

fn main() {
    let mut t = BinTree::new();
    // t.add_sorted(4);
    // t.add_sorted(5);
    // t.add_sorted(6);
    // t.add_sorted(10);
    // t.add_sorted(1);
    // t.add_sorted(94);
    // t.add_sorted(54);
    // t.add_sorted(3);

    for i in 0..100 {
        t.add_sorted(i);
    }

    t.print_left_first(0);

    // println!("-------------------------");
    // t.rot_left();
    // // t.rot_right();
    // t.print_left_first(0);

    // println!("t = {:?}", t);
}
