use std::fmt::Debug;
use flume::Receiver;
use winit::event_loop::EventLoopProxy;

pub trait Sink<T> {
    async fn send(&self, data: T) -> bool;
}

impl<T: Debug> Sink<T> for EventLoopProxy<T> {
    async fn send(&self, data: T) -> bool {
        self.send_event(data)
            .map_err(|e| log::warn!("Eventloop closed. Dropping {:?}.", e.0))
            .is_ok()
    }
}

pub trait Source<T> {
    async fn recv(&self) -> Option<T>;
}

impl<T> Source<T> for Receiver<T> {
    async fn recv(&self) -> Option<T> {
        self.recv_async().await.ok()
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
