use std::arch::asm;

// Inlining is when the compiler omits the function call and simply copies 
// the body of the function instead of calling it.
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        asm!(
            // puts the value 1 in the rax register
            // When the CPU traps our call later on and passes control to the OS,
            // the kernel knows that a value of 1 in rax means that we want to make a write.
            "mov rax, 1",
            // A value of 1 in rdi means that we want to write to stdout.
            "mov rdi, 1",
            // This instruction issues a software interrupt, and the CPU passes on control to the OS.
            "syscall",
            // writes the address to the buffer where our text is stored in the rsi register
            in("rsi") msg_ptr,
            in("rdx") len,
            // The next four lines are not instructions to the CPU; they're meant to tell the compiler
            // that it cann't store anything in these registers and assume the data is untouched when we
            // exit the inline assembly block. We do that by telling the compiler that there will be some
            // unspecified data written to these registers.
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}

fn main() {
    let message = "Hello world from raw syscall!\n".to_string();
    syscall(message);
}
