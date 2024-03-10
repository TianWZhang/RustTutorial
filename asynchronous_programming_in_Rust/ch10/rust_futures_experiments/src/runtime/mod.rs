mod executor;
mod reactor;

pub use executor::{Executor, spawn};

pub fn init() -> Executor {
    reactor::start();
    Executor::new()
}
