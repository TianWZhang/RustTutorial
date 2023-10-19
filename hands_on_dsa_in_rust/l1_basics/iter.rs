pub struct Stepper {
    cur: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if(self.cur >= max) {
            return None;
        }
        let res = self.cur;
        self.cur += self.step;
        Some(res)
    }
}

fn main() {
    let st = Stepper{cur: 2, step: 3, max: 15};
    loop {
        match st.next() {
            Some(v) => println!("loop {}", v),
            None => break
        }
    }
    
    for i in Stepper{cur: 5, step: 10, max: 50} {
    	println!("for loop {}, i);
    }
}
