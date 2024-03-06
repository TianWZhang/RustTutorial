#![feature(naked_functions)]
use std::arch::asm;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2; // 2M
const MAX_THREADS: usize = 5;
static mut RUNTIME: *mut Runtime = std::ptr::null_mut();

/// main entry point
pub struct Runtime {
    threads: Vec<GreenThread>,
    /// which thread we are currently running
    current: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    /// Available means the thread is available and ready to be assigned a task if needed.
    Available,
    Running,
    // Ready means the thread is ready to move forward and resume execution.
    Ready,
}

struct GreenThread {
    id: usize,
    stack: Box<[u8]>,
    ctx: ThreadContext,
    state: State,
    task: Option<Box<dyn FnOnce()>>,
}

impl GreenThread {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            stack: Box::from([0u8; DEFAULT_STACK_SIZE]),
            ctx: ThreadContext::default(),
            state: State::Available,
            task: None,
        }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
/// ABI states that callee (which will be our switch function from the perspective of the OS)
/// needs to restore them before the caller is resumed.
/// ThreadContext holds data for the registers that the CPU needs to resume execution on a stack.
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    thread_ptr: u64
}

impl Runtime {
    pub fn new() -> Self {
        let main_thread = GreenThread {
            id: 0,
            stack: Box::from([0u8; DEFAULT_STACK_SIZE]),
            ctx: ThreadContext::default(),
            state: State::Running,
            task: None,
        };
        let mut threads = vec![main_thread];
        threads[0].ctx.thread_ptr = &threads[0] as *const GreenThread as u64;
        let other_threads = (1..MAX_THREADS).map(|i| GreenThread::new(i));
        threads.extend(other_threads);
        Runtime {
            threads,
            current: 0,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            if RUNTIME.is_null() {
                RUNTIME = self as *mut Runtime;
            }
        }
    }

    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        // When t_yield() returns false, there is no more work to do and we can exit the process.
        std::process::exit(0);
    }

    // The user of spawned threads does not call this; we set up our stack so this is called
    // when the task is done. In fact, the only place where this function is called is on
    // the `guard` function.
    fn t_return(&mut self) {
        // If the calling thread is the main_thread, we won't do anything. Our runtime will call t_yield()
        // for us on the main_thread (in the run() function).
        if self.current != 0 {
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        // go through all the threads and see if any are ready to make progress
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                // no thread is Ready
                return false;
            }
        }
        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }
        self.threads[pos].state = State::Running;

        let old_pos = self.current;
        self.current = pos;
        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;
            // The clobber_abi("C") argument tells the compiler that it may not assume that any
            // general-purpose registers are preserved across the asm! block. The compiler will
            // emit instructions to push the registers it uses to the stack and restore them when
            // resuming after the asm! block.
            // When calling a normal function, the cimpiler will insert code to save/restore all the
            // caller-saved registers before calling a function so it can resume with the correct
            // state when the function returns. Since we marked `switch` as naked, we explicitly told
            // the compiler to not insert this code, so the safest thing is to make sure the compiler
            // doesn't assume that it can rely on any register being untouched when it resumes after the call
            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
        }
        // just a way to prevent the compiler from optimizing our code away
        self.threads.len() > 0
    }

    pub fn spawn<F: FnOnce() + 'static>(f: F) {
        unsafe {
            let available = (*RUNTIME)
                .threads
                .iter_mut()
                .find(|t| t.state == State::Available)
                .expect("no available green thread");

            // stack bottom
            let s_ptr = available
                .stack
                .as_mut_ptr()
                .offset(available.stack.len() as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            available.task = Some(Box::new(f));
            available.ctx.thread_ptr = available as *const GreenThread as u64;
            // We write the address to our guard function that will be called when the task we provide finishes
            // and the function returns.
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            // just to handle the gap when we return from f so that `guard` will get called on a 16-byte boundary
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, call as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
            available.state = State::Ready;
        }
    }
}

fn guard() {
    unsafe {
        let rt = &mut *RUNTIME;
        println!("THREAD {} FINISHED.", rt.threads[rt.current].id);
        (*RUNTIME).t_return();
    }
}

// Tells the compiler that we don't want it to create a function prologue and epilogue
// and that we want to take care of this ourselves.
#[naked]
unsafe extern "C" fn skip() {
    // ret will pop off the next value from the stack and jump to whatever instructions that
    // address points to, which is `guard` in our case
    asm!("ret", options(noreturn))
}

fn call(thread: u64) {
    let thread = unsafe { &mut *(thread as *mut GreenThread) };
    if let Some(f) = thread.task.take() {
        f();
    }
}

pub fn yield_thread() {
    unsafe {
        (*RUNTIME).t_yield();
    }
}

#[naked]
#[no_mangle]
// Naked functions don't accept formal arguments, hence
// we can't simply call it in Rust as a normal function.
// When we call a function with two arguments, the compiler
// will place each argument in a register described by the
// calling convention for the platform. However, when we call
// a naked function, we need to take care of this ourselves.
unsafe extern "C" fn switch() {
    asm!(
        "mov [rdi + 0x00], rsp",
        "mov [rdi + 0x08], r15",
        "mov [rdi + 0x10], r14",
        "mov [rdi + 0x18], r13",
        "mov [rdi + 0x20], r12",
        "mov [rdi + 0x28], rbx",
        "mov [rdi + 0x30], rbp",
        "mov rsp, [rsi + 0x00]",
        "mov r15, [rsi + 0x08]",
        "mov r14, [rsi + 0x10]",
        "mov r13, [rsi + 0x18]",
        "mov r12, [rsi + 0x20]",
        "mov rbx, [rsi + 0x28]",
        "mov rbp, [rsi + 0x30]",
        // place the first argument of `call` in the rdi register
        "mov rdi, [rsi + 0x38]",
        "ret",
        options(noreturn)
    );
}

// override the default toolchain for the entire current directory by writing
// rustup override set nightly
fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    Runtime::spawn(|| {
        println!("Thread 1 starting");
        for i in 0..10 {
            println!("thread 1 counter: {}", i);
            yield_thread();
        }
    });
    Runtime::spawn(|| {
        println!("Thread 2 starting");
        for i in 0..15 {
            println!("thread 2 counter: {}", i);
            yield_thread();
        }
    });
    Runtime::spawn(|| {
        println!("We can nest tasks...");
        Runtime::spawn(|| {
            println!("...like this!");
        })
    });
    runtime.run();
}
