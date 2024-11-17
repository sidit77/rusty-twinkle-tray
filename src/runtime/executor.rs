use std::cell::Cell;
use std::collections::VecDeque;
use std::future::{poll_fn, Future};
use std::mem::take;
use std::pin::Pin;
use std::task::Poll;
use futures_lite::pin;

type Task = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// A simple executor that runs tasks on the current thread.
///
/// It doesn't scale very well with many tasks, but it's good enough for our use case
/// as we are only spawning one task per monitor and people *hopefully* do not have thousands of monitors.
#[derive(Default)]
pub struct LocalExecutor {
    tasks: Cell<VecDeque<Task>>
}

impl LocalExecutor {

    fn queue_task(&self, task: Task) {
        let mut tasks = self.tasks.take();
        tasks.push_back(task);
        self.tasks.set(tasks);
    }

    pub async fn run<T>(&self, fut: impl Future<Output = T>) -> T {
        pin!(fut);

        let mut local_buffer = VecDeque::new();

        poll_fn(|cx| {
            local_buffer = self.tasks.replace(take(&mut local_buffer));
            while let Some(mut task) = local_buffer.pop_front() {
                match task.as_mut().poll(cx) {
                    Poll::Ready(()) => { /* task done; don't requeue */ },
                    Poll::Pending => self.queue_task(task),
                }
            }
            fut.as_mut().poll(cx)
        }).await

    }

    pub fn spawn(&self, fut: impl Future<Output = ()> + 'static) {
        self.queue_task(Box::pin(fut));
    }

}