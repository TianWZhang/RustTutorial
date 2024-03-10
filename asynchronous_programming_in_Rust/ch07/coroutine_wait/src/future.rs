pub trait Future {
    type Output;
    fn poll(&mut self) -> PollState<Self::Output>;
}

pub enum PollState<T> {
    Ready(T),
    Pending,
}

pub struct JoinAll<F: Future> {
    futures: Vec<(bool, F)>,
    finished_count: usize
}

impl<F: Future> Future for JoinAll<F> {
    type Output = String;
    fn poll(&mut self) -> PollState<Self::Output> {
        // loop over each (flag, future) tuple
        for (finished, fut) in self.futures.iter_mut() {
             if *finished {
                continue;
             }
             // The first time JoinAll::poll is called, it will call poll on each
             // future in the collection, which will kick off whatever operation 
             // they represent and allow them to progress concurrently.
             // This is one way to achieve concurrency with lazy coroutines.
             match fut.poll() {
                PollState::Ready(_) => {
                    *finished = true;
                    self.finished_count += 1;
                }
                PollState::Pending => continue
             }
        }
        // After iterating throught the entire collection, we check if we've 
        // resolved all the futures we originally received.
        if self.finished_count == self.futures.len() {
            PollState::Ready(String::new())
        } else {
            PollState::Pending
        }
    }
}

pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    let futures = futures.into_iter().map(|f| (false, f)).collect();
    JoinAll {
        futures,
        finished_count: 0
    }
}
