use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinNode<T>>>);

#[derive(Debug)]
pub struct BinNode<T> {
    data: T,
    h: i8,//height
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinNode<T> {
    fn rot_left(mut self) -> Box<Self> {
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), //without right node we cannot rotate
        };

        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }

    fn rot_right(mut self) -> Box<Self> {
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), //without right node we cannot rotate
        };

        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0
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
        let rot_direction = match self.0 {
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
                self.0 = Some(Box::new(BinNode {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new()
                }));
                0
            }
        };
        match rot_direction {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
        
    }
}

impl<T: Debug> BinTree<T> { 
    pub fn print_lfirst(&self, depth: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(depth + 1);
            let mut spc = String::new();
            for _ in 0..depth {
                spc.push('.');
            }
            println!("{}: {}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst(depth + 1);
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

    for i in 0..100000 {
        t.add_sorted(i);
    }
    t.print_lfirst(0);
}
