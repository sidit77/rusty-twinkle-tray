use std::fmt::Debug;
use std::future::Future;
use flume::Sender;
use winit::event_loop::EventLoopProxy;

pub trait Sink<T> {
    fn send(&self, data: T) -> impl Future<Output = bool>;
}

impl<T: Debug> Sink<T> for Sender<T> {
    async fn send(&self, data: T) -> bool {
        self.send_async(data)
            .await
            .map_err(|e| log::warn!("Eventloop closed. Dropping {:?}.", e.0))
            .is_ok()
    }
}

pub trait SinkExt<T> {
    fn map<U, F: Fn(U) -> T>(self, func: F) -> Map<Self, F> where Self: Sized;
}

impl<T, S: Sink<T>> SinkExt<T> for S {
    fn map<U, F: Fn(U) -> T>(self, func: F) -> Map<Self, F> {
        Map {
            sink: self,
            func,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Map<S, F> {
    sink: S,
    func: F
}

impl<U, T, F: Fn(U) -> T, S: Sink<T>> Sink<U> for Map<S, F> {
    async fn send(&self, data: U) -> bool {
        self.sink.send((self.func)(data)).await
    }
}
