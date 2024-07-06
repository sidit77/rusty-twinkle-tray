use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::mem::replace;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

#[derive(Default)]
struct TimerState {
    timers: BTreeMap<(Instant, usize), Waker>,
    next_id: usize
}

impl TimerState {
    pub fn insert_timer(&mut self, at: Instant, waker: Waker) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.timers.insert((at, id), waker);
        id
    }

    pub fn remove_timer(&mut self, at: Instant, id: usize) {
        self.timers.remove(&(at, id));
    }

    pub fn process_timers(&mut self) -> Option<Duration> {
        let now = Instant::now();

        let pending = self.timers.split_off(&(now + Duration::from_nanos(1), 0));
        let ready = replace(&mut self.timers, pending);

        ready.values().for_each(Waker::wake_by_ref);

        self.timers
            .keys()
            .next()
            .map(|(when, _)| when.saturating_duration_since(now))
    }
}

thread_local! { static LOCAL_STATE: RefCell<TimerState> = RefCell::new(TimerState::default()); }

pub fn process_timers_for_current_thread() -> Option<Duration> {
    LOCAL_STATE.with(|timers| timers.borrow_mut().process_timers())
}

pub struct Timer {
    at: Option<Instant>,
    id: Option<usize>
}

impl Timer {
    pub const fn never() -> Self {
        Self { at: None, id: None }
    }

    pub const fn at(instant: Instant) -> Self {
        Self { at: Some(instant), id: None }
    }

    pub fn after(duration: Duration) -> Self {
        Instant::now()
            .checked_add(duration)
            .map_or_else(Self::never, Self::at)
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.at {
            None => Poll::Pending,
            Some(at) => match Instant::now() >= at {
                true => Poll::Ready(()),
                false => {
                    if self.id.is_none() {
                        self.id = LOCAL_STATE
                            .with(|timers| timers.borrow_mut().insert_timer(at, cx.waker().clone()))
                            .into();
                    }
                    Poll::Pending
                }
            }
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if let Some((at, id)) = self.at.zip(self.id) {
            LOCAL_STATE.with(|timers| {
                timers.borrow_mut().remove_timer(at, id);
            });
        }
    }
}
