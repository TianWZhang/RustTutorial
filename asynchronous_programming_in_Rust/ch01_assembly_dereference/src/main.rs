use std::arch::asm;

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t;
    let x = dereference(t_ptr);
    println!("{}", x);
}

fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        // The mov instruction instructs the CPU to take the first 8 bytes it gets when reading the memory 
        // location that {1} points to and place that in the register represented by {0}.
        // The [] will instruct the CPU to treat the data in that register as a memory address, and instead of
        // simply copying the memory address itself to {0}, it will fetch what's at that memory and move it over.
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}
