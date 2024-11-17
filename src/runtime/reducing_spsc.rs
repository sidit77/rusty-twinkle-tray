use std::cell::Cell;
use std::error::Error;
use std::fmt::Display;
use std::future::{poll_fn, Future};
use std::rc::Rc;
use std::task::{Poll, Waker};


pub trait Reducible {
    fn reduce(self, other: Self) -> Self;

}

pub fn channel<T: Reducible>() -> (ReducingSender<T>, ReducingReceiver<T>) {
    let shared = Rc::new(Shared {
        value: Cell::new(None),
        closed: Cell::new(false),
        waiter: Cell::new(None)
    });

    (ReducingSender(shared.clone()), ReducingReceiver(shared))
}

struct Shared<T> {
    value: Cell<Option<T>>,
    closed: Cell<bool>,
    waiter: Cell<Option<Waker>>
}


pub struct ReducingSender<T>(Rc<Shared<T>>);

impl<T: Reducible> ReducingSender<T> {
    pub fn try_send(&self, value: T) -> Result<(), TrySendError> {
        if self.0.closed.get() {
            return Err(TrySendError::Closed);
        }

        self.0.value.set(match self.0.value.take() {
            None => Some(value),
            Some(current) => Some(current.reduce(value))
        });

        if let Some(waker) = self.0.waiter.take() {
            waker.wake();
        }

        Ok(())
    }
}

impl<T> Drop for ReducingSender<T> {
    fn drop(&mut self) {
        self.0.closed.set(true);
        if let Some(waker) = self.0.waiter.take() {
            waker.wake();
        }
    }
}

pub struct ReducingReceiver<T>(Rc<Shared<T>>);

impl<T> ReducingReceiver<T> {
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        match self.0.value.take() {
            None => match self.0.closed.get() {
                true => Err(TryRecvError::Closed),
                false => Err(TryRecvError::Empty)
            },
            Some(v) => Ok(v)
        }
    }

    pub fn recv(&mut self) -> impl Future<Output = Option<T>> + '_{
        poll_fn(move |cx| {
            match self.try_recv() {
                Ok(v) => Poll::Ready(Some(v)),
                Err(TryRecvError::Closed) => Poll::Ready(None),
                Err(TryRecvError::Empty) => {
                    self.0.waiter.replace(Some(cx.waker().clone()));
                    Poll::Pending
                }
            }
        })
    }

}

impl<T> Drop for ReducingReceiver<T> {
    fn drop(&mut self) {
        self.0.closed.set(true);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrySendError {
    Closed
}

impl Display for TrySendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Channel is closed")
    }
}

impl Error for TrySendError {}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TryRecvError {
    Closed,
    Empty
}

impl Display for TryRecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TryRecvError::Closed => write!(f, "Channel is closed"),
            TryRecvError::Empty => write!(f, "Channel is empty")
        }
    }
}

impl Error for TryRecvError {}