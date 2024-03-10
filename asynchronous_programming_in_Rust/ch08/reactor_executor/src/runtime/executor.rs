use crate::future::{Future, PollState};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, Thread}
};

type Task = Box<dyn Future<Output = String>>;
// Define a thread-local static variable that's unique to the thread it's first called from.
// All the threads we create will have their own instance, and it's impossible for one thread
// to access another thread's CURRENT_EXEC variable.
thread_local! {
    static CURRENT_EXEC: ExecutorCore = ExecutorCore::default();
}

#[derive(Default)]
struct ExecutorCore {
    // holds all the top-level futures associated with the executor on this thread
    // We cann't simply mutate a static variable, so we need internal mutability here.
    // A RefCell will do so since this will only be callable from one thread and there is 
    // no need for synchronization.
    tasks: RefCell<HashMap<usize, Task>>,
    // stores the IDs of tasks that should be polled by the executor
    // A shared reference to this collection will be given to each Waker that this executor creates.
    // The Waker will be sent to different thread and signal that a specific task is ready by adding 
    // the task ID to ready_queue.
    ready_queue: Arc<Mutex<Vec<usize>>>,
    next_id: Cell<usize>
}

pub fn spawn<F>(future: F)
// The lifetime of F must be able to last until the end of the program.
where 
    F: Future<Output = String> + 'static
{
    CURRENT_EXEC.with(|e| {
        let id = e.next_id.get();
        e.tasks.borrow_mut().insert(id, Box::new(future));
        e.ready_queue.lock().map(|mut q|q.push(id)).unwrap();
        e.next_id.set(id + 1);
    });
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    fn pop_ready(&self) -> Option<usize> {
        CURRENT_EXEC.with(|e| e.ready_queue.lock().map(|mut q| q.pop()).unwrap())
    }

    fn get_future(&self, id: usize) -> Option<Task> {
        CURRENT_EXEC.with(|e| e.tasks.borrow_mut().remove(&id))
    }

    fn get_waker(&self, id: usize) -> Waker {
        Waker {
            id,
            thread: thread::current(),
            ready_queue: CURRENT_EXEC.with(|e| e.ready_queue.clone())
        }
    }

    fn insert_task(&self, id: usize, task: Task) {
        CURRENT_EXEC.with(|e| e.tasks.borrow_mut().insert(id, task));
    }

    fn task_count(&self) -> usize {
        CURRENT_EXEC.with(|e| e.tasks.borrow().len())
    }

    // Entry point to the Executor.
    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = String> + 'static
    {
        spawn(future);
        loop {
            while let Some(id) = self.pop_ready() {
                let mut future = match self.get_future(id) {
                    Some(f) => f,
                    // guard against false wakeups
                    None => continue
                };
                let waker = self.get_waker(id);
                match future.poll(&waker) {
                    PollState::Pending => self.insert_task(id, future),
                    PollState::Ready(_) => continue
                }
            }
            let task_count = self.task_count();
            let name = thread::current().name().unwrap_or_default().to_string();
            if task_count > 0 {
                println!("{}: {} pending tasks. Sleep until notified.", name, task_count);
                // Parking the thread will yield control to the OS scheduler, and the Executor
                // does nothing until it's woken up again.
                thread::park();
            } else {
                println!("{}: All tasks are finished", name);
                break;
            }
        }
    }
}

#[derive(Clone)]
pub struct Waker {
    thread: Thread,
    /// which task this Waker is associated with
    id: usize,
    /// A reference that can be shared between threads to a Vec<usize>, 
    /// where usize represents the task ID.
    ready_queue: Arc<Mutex<Vec<usize>>>,
}

impl Waker {
    pub fn wake(&self) {
        self.ready_queue
            .lock()
            // push the task id that this Waker is associated with onto the ready queue
            .map(|mut q| q.push(self.id))
            .unwrap();
        self.thread.unpark();
    }
}
