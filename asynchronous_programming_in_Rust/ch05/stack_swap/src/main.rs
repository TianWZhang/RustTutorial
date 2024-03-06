use core::arch::asm;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    // rsp is the register that stores the stack pointer that
    // the CPU uses to figure out the current location on the stack
    rsp: u64
}

fn hello() -> ! {
    println!("I love waking up on a new stack!");
    loop {}
}

// If we want the CPU to swap to a different stack, we need to set rsp to the top 
// of our new stack and set the instruction pointer(rip) to point to the address `hello`.
// If we can manipulate it directly, the CPU would fetch the instruction pointed to by the rip
// register and execute the first instruction we wrote in our `hello` function. However, 
// there is no way for us to manipulate rip directly on the x86-64 instruction set.
unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        // moves the value stored at 0x00 offset from the memory location at {0} to rsp
        // rsp usually stores a pointer to the most recently pushed value on the stack
        "mov rsp, [{0} + 0x00]",
        // pops a memory location off the stack and then makes an unconditional jump to that location
        "ret",
        // the 1st non-assembly argument to the asm! macro
        // When we write in(reg), we let the compiler decide on a general-purpose register to 
        // store the value of `new`.
        in(reg) new,
    );
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        // rounds our memory address down to the nearest 16-byte-aligned address
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;

        for i in 0..SSIZE {
            println!(
                "mem: {}, val: {}",
                sb_aligned.offset(-i as isize) as usize,
                *sb_aligned.offset(-i as isize)
            )
        }

        gt_switch(&mut ctx);
    }
}
