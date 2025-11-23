use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{current, park, park_timeout, Thread};

pub use executor::LocalExecutor;
use futures_lite::{pin, Stream};
pub use timer::{process_timers_for_current_thread, Timer};

mod executor;
pub mod reducing_spsc;
mod timer;

#[derive(Default)]
pub enum FutureStream<T> {
    #[default]
    Empty,
    Waiting(Pin<Box<dyn Future<Output = Option<T>>>>)
}

impl<T> FutureStream<T> {
    pub const fn new() -> Self {
        Self::Empty
    }

    pub fn clear(&mut self) {
        *self = FutureStream::Empty;
    }

    pub fn set<F>(&mut self, future: F)
    where
        F: Future<Output = Option<T>> + 'static
    {
        *self = FutureStream::Waiting(Box::pin(future));
    }
}

impl<T> Stream for FutureStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.as_mut().get_mut() {
            FutureStream::Waiting(fut) => match fut.as_mut().poll(cx) {
                Poll::Ready(result) => {
                    self.set(FutureStream::Empty);
                    match result {
                        Some(item) => Poll::Ready(Some(item)),
                        None => Poll::Pending
                    }
                }
                Poll::Pending => Poll::Pending
            },
            FutureStream::Empty => Poll::Pending
        }
    }
}


struct Signal {
    thread: Thread,
    signaled: AtomicBool
}

impl Signal {
    fn new() -> Self {
        Self {
            thread: current(),
            signaled: AtomicBool::new(false)
        }
    }

    fn ready(&self) -> bool {
        self.signaled.load(Ordering::SeqCst)
    }

    fn reset(&self) {
        self.signaled.store(false, Ordering::SeqCst)
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.signaled.store(true, Ordering::SeqCst);
        self.thread.unpark();
    }
}

pub fn block_on<F: Future>(fut: F) -> F::Output {
    pin!(fut);

    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    let mut context = Context::from_waker(&waker);

    loop {
        signal.reset();
        match fut.as_mut().poll(&mut context) {
            Poll::Pending => {
                let sleep_dur = process_timers_for_current_thread();
                match signal.ready() {
                    true => continue,
                    false => match sleep_dur {
                        None => park(),
                        Some(dur) => park_timeout(dur)
                    }
                }
            }
            Poll::Ready(item) => break item
        }
    }
}
