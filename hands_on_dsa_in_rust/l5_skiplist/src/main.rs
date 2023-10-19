use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{self, Debug, Write};

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        Self {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>>{
        // bigger than right then go right
        if let Some(ref mut rt) = self.right {
            if dt > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(dt);
            }
        } 

        // has lower children try them
        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        let dt = child.borrow().data.clone();
                        let nn = SkipNode {
                            right: self.right.take(),
                            data: dt,
                            down: Some(child)
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    },
                    false => None
                }
                None => None
            };
        }
        // should be before right, at bottom node
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }

    pub fn find(&self, target: &T) -> bool {
        if *target == *self.data.borrow() {
            return true;
        }

        // has right node and target >= right data => try right node
        if let Some(ref rt) = self.right {
            if *target >= *rt.borrow().data.borrow() {
                return rt.borrow().find(target);
            }
        } 

        // has lower children try them
        if let Some(ref dw) = self.down {
            return dw.borrow().find(target) ;
        }
        false
    }

    // *target must be larger than *self.data.borrow()
    fn delete(&mut self, target: &T) {
        // has right node and target >= right data => try right node
        if let Some(ref rt) = self.right.take() {
            if *target == *rt.borrow().data.borrow() {
                self.right = rt.borrow_mut().right.take();
            } else if *target > *rt.borrow().data.borrow(){
                self.right = Some(rt.clone());
                rt.borrow_mut().delete(target);
                return;
            } else {
                self.right = Some(rt.clone());
            }
        } 

        // has lower children try them
        if let Some(ref dw) = self.down {
            dw.borrow_mut().delete(target) ;
        }
    }
}

#[derive(Debug)]
pub struct SkipList<T: PartialOrd>(Vec<SkipNode<T>>);

impl<T: PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    pub fn insert(&mut self, data: T) {
        if self.0.len() == 0 {
            self.0.push(SkipNode::new(data));
            return;
        }
        // Our vec will have the lowest row, with the lowest number
        // we need to try and insert in the highest available row
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(child) = self.0[i].insert(data) {
                    self.loop_up(child, i+1);
                }
                return;
            }
        }
        // if none of those succeeded, that means we have an element to replace the first
        let mut nn = SkipNode::new(data);
        // put our new element on the front of the row
        std::mem::swap(&mut nn, &mut self.0[0]);

        let res = rcc(nn);
        self.0[0].right = Some(res.clone());
        self.loop_up(res, 1);
    }

    pub fn loop_up(&mut self, ch: Rcc<SkipNode<T>>, n: usize) {
        if rand::random::<bool>() {
            return;
        }
        let dt = ch.borrow().data.clone();
        let mut nn = SkipNode {
            right: None,
            down: Some(ch),
            data: dt,
        };
        if n >= self.0.len() {
            self.0.push(nn);
            return;
        }

        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rcc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n + 1);
    }

    pub fn find(&self, target: &T) -> bool {
        for i in (0..self.0.len()).rev() {
            if target >= & *self.0[i].data.borrow() {
                return self.0[i].find(target);
            }
        }
        false
    }

    pub fn delete(&mut self, target: &T) {
        if !self.find(target) {return;}
        for i in (0..self.0.len()).rev() {
            if *target > *self.0[i].data.borrow()  {
                self.0[i].delete(target);
                return;
            } else if *target == *self.0[i].data.borrow() {
                if let Some(rt) = self.0[i].right.clone() {
                    std::mem::swap(&mut *rt.borrow_mut(), &mut self.0[i]);
                } else {
                    self.0.remove(i);
                }
            }
        }
    }
}

impl<T: Debug + PartialOrd> SkipNode<T> {
    pub fn print_row<W: Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref r) = self.right {
            write!(w, ",")?;
            r.borrow().print_row(w)?;
        }
        Ok(())
    }
}

impl<T: Debug + PartialOrd> fmt::Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "SkipList is empty");
        }

        for sn in &self.0 {
            write!(f, "\n")?;
            sn.print_row(f)?;
        }
        Ok(())
    }
}


fn main() {
    let mut s = SkipList::new();
    s.insert(4);
    s.insert(6);
    s.insert(77);
    s.insert(84);
    s.insert(23);
    s.insert(1);
    s.insert(65);
    s.insert(18);
    assert!(s.find(&77));
    assert!(!s.find(&83));
    println!("s = {}", s);
    s.delete(&18);
    s.delete(&1);
    s.delete(&4);
    println!("s = {}", s);
}