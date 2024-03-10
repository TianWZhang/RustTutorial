use crate::runtime::Waker;
use mio::{net::TcpStream, Events, Interest, Poll, Registry, Token};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, OnceLock
    },
    thread
};

type Wakers = Arc<Mutex<HashMap<usize, Waker>>>;

// This will be possible to access from different threads.
// OnceLock allows us to define a static variable that we can
// write to once so that we can initialize it when we start
// our Reactor.
static REACTOR: OnceLock<Reactor> = OnceLock::new();

pub fn reactor() -> &'static Reactor {
    REACTOR.get().expect("Called outside an runtime context")
}

pub struct Reactor {
    wakers: Wakers,
    /// to interact with the event queue in mio
    registry: Registry,
    /// Stores the next available ID so that we can track which 
    /// event occurred and which Waker should be woken.
    next_id: AtomicUsize
}

impl Reactor {
    // We pass in and id so that we can identify which event has occurred when 
    // we receive a notification later on.
    pub fn register(&self, stream: &mut TcpStream, interest: Interest, id: usize) {
        self.registry.register(stream, Token(id), interest).unwrap();
    }

    pub fn set_waker(&self, waker: &Waker, id: usize) {
        let _ = self
            .wakers
            .lock()
            .map(|mut w| w.insert(id, waker.clone()).is_none())
            .unwrap();
    }

    pub fn deregister(&self, stream: &mut TcpStream, id: usize) {
        self.wakers.lock().map(|mut w| w.remove(&id)).unwrap();
        self.registry.deregister(stream).unwrap();
    }

    pub fn next_id(&self) -> usize {
        // We don't care about any happens before/after relationships happening here; 
        // we only care about not handing out the same value twice.
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}

fn event_loop(mut poll: Poll, wakers: Wakers) {
    let mut events = Events::with_capacity(100);
    // continue to loop for eternity
    loop {
        poll.poll(&mut events, None).unwrap();
        for e in events.iter() {
            let Token(id) = e.token();
            let wakers = wakers.lock().unwrap();
            // Waker may have been removed, in which case we do nothing.
            if let Some(waker) = wakers.get(&id) {
                waker.wake();
            }
        }
    }
}

pub fn start() {
    let wakers = Arc::new(Mutex::new(HashMap::new()));
    let poll = Poll::new().unwrap();
    let registry = poll.registry().try_clone().unwrap();
    let next_id = AtomicUsize::new(1);
    let reactor = Reactor {
        wakers: wakers.clone(),
        registry,
        next_id
    };
    REACTOR.set(reactor).ok().expect("Reactor already running");
    thread::spawn(move || event_loop(poll, wakers));
}
