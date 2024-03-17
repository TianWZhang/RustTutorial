use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[allow(dead_code)]
pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    let futures = futures.into_iter().map(|f| (false, Box::pin(f))).collect();
    JoinAll {
        futures,
        finished_count: 0,
    }
}

pub struct JoinAll<F: Future> {
    futures: Vec<(bool, Pin<Box<F>>)>,
    finished_count: usize,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self {
            futures,
            finished_count,
        } = &mut *self;
        for (finished, fut) in futures.iter_mut() {
            if *finished {
                continue;
            }

            match fut.as_mut().poll(cx) {
                Poll::Ready(_) => {
                    *finished = true;
                    *finished_count += 1;
                }
                Poll::Pending => continue,
            }
        }
        if self.finished_count == self.futures.len() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
