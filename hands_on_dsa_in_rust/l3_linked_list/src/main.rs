use std::cell::{RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>
}

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // create a new front object
                let new_front = Rc::new(RefCell::new(DbNode {
                    data, 
                    next: Some(r.clone()), 
                    prev: None})
                );
                // tell the first object this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // put this on the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {data, next: None, prev: None}));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
                
            } 
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // create a new back object
                let new_back = Rc::new(RefCell::new(DbNode {
                    data, 
                    prev: Some(r.clone()), 
                    next: None})
                );
                // put this on the back
                self.last = Some(Rc::downgrade(&new_back));

                // tell the last object this is now behind it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {data, next: None, prev: None}));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
                
            } 
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);
    println!("ll = {:?}", ll);


    let mut dl = DbList::new();
    dl.push_front(6);
    dl.push_back(11);
    dl.push_front(5);
    dl.push_back(15);
    dl.push_front(4);
    println!("dl = {:?}", dl);
}