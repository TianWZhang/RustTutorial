use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
    thread,
};

fn f1(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("changed");
    }
}

#[allow(dead_code)]
fn f2(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

#[allow(dead_code)]
fn f3(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    // Both the original and cloned Rc will refer to the same allocation; they
    // share ownership.
    assert_eq!(a.as_ptr(), b.as_ptr());

    let a = Arc::new([1, 2, 3]);
    let t = thread::spawn({
        // increments the reference count to two
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    dbg!(a);
    t.join().unwrap();

    let a = Cell::new(2);
    f1(&a, &a);
}
