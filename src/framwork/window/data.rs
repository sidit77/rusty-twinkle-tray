use std::cell::Cell;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use futures_lite::Stream;
use super::super::buffer::CircularBuffer;
use super::Event;

#[derive(Default)]
pub struct WindowData {
    queue: CircularBuffer<Event, 32>,
    current: Cell<Option<Waker>>
}

impl WindowData {

    pub fn push_event(&self, event: Event) {
        println!("pushing: {:?}", event);
        while !self.queue.try_push(event) {
            self.queue.try_pop();
        }
        if let Some(waker) = self.current.take() {
            waker.wake_by_ref();
            self.current.set(Some(waker));
        }
    }

    pub fn stream(&self) -> WindowEvents<'_> {
        WindowEvents {
            inner: self,
        }
    }
    
}

pub struct WindowEvents<'a> {
    inner: &'a WindowData
}

impl<'a> Stream for WindowEvents<'a> {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let data = self.inner;
        match data.queue.try_pop() {
            Some(event) => Poll::Ready(Some(event)),
            None => {
                data.current.set(Some(cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}