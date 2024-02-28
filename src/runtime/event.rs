use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

#[derive(Default)]
pub struct Event {
    signaled: Cell<bool>,
    waker: Cell<Option<Waker>>
}

impl Event {

    pub fn reset(&self) {
        self.signaled.set(false);
        self.waker.set(None);
    }

    pub fn signal(&self) {
        self.signaled.set(true);
        if let Some(waker) = self.waker.take() {
            waker.wake_by_ref();
            self.waker.set(Some(waker));
        }
    }

    pub fn wait(&self) -> WaitFut<'_> {
        WaitFut {
            event: self,
            first: true,
        }
    }

}

pub struct WaitFut<'a> {
    event: &'a Event,
    first: bool
}

impl<'a> Future for WaitFut<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.event.signaled.get() {
            self.event.waker.set(None);
            Poll::Ready(())
        } else {
            let prev = self.event.waker.replace(Some(cx.waker().clone()));
            assert!(prev.is_none() || !self.first, "Another waker was already registered");
            self.first = false;
            Poll::Pending
        }
    }
}

impl<'a> Drop for WaitFut<'a> {
    fn drop(&mut self) {
        self.event.waker.set(None);
    }
}
