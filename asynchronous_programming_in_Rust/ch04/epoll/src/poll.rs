use crate::ffi;
use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
};

type Events = Vec<ffi::Event>;

/// a struct that represents the event queue
pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let res = unsafe { ffi::epoll_create(1) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            registry: Registry { raw_fd: res },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thread it's called on until an event is ready or it times out, whichever occurs first
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        // ffi::epoll_wait will return successfully if it has a value of 0 or larger, telling us how many
        // events have occured
        let res = unsafe { ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe { events.set_len(res as usize) };
        Ok(())
    }
}

// The responsibility of registering events and the queue itself is divided. By moving the concern of registering
// interests to a separate struct, users can call Registry::try_clone to get an owned Registry instance. This instance
// can be passed to or shared with other threads, allowing multiple threads to register interest to the same Poll
// instance even when Poll is blocking another thread while waiting for new events to happen in Poll::poll (it requires exclusive access).
// This design lets users interact with the queue with potentially many threads by registering interest, while one thread
// makes the blocking call and handles the notifications from the OS.
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    // interests is a bitmask
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };
        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(
                self.raw_fd,
                op,
                source.as_raw_fd(), //the fd we want to queue to track
                &mut event, // to indicate what kind of events we're interested in getting notifications for
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { ffi::close(self.raw_fd) };
        if res < 0 {
            let e = io::Error::last_os_error();
            eprintln!("ERROR: {:?}", e);
        }
    }
}
