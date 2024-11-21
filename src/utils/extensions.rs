use std::sync::{Mutex, MutexGuard};
use crate::runtime::reducing_spsc::{Reducible, ReducingSender};

pub trait ChannelExt<T> {

    #[track_caller]
    fn filter_send_ignore(&self, msg: Option<T>){
        if let Some(msg) = msg {
            self.send_ignore(msg);
        }
    }
    fn send_ignore(&self, msg: T);
}

impl<T> ChannelExt<T> for loole::Sender<T> {

    #[track_caller]
    fn send_ignore(&self, msg: T) {
        self.send(msg)
            .unwrap_or_else(|err| log::warn!("Failed to send message: {}", err));
    }
}

impl<T: Reducible> ChannelExt<T> for ReducingSender<T> {

    #[track_caller]
    fn send_ignore(&self, msg: T) {
        self.try_send(msg)
            .unwrap_or_else(|err| log::warn!("Failed to send message: {:?}", err));
    }
}

pub trait FunctionalExt: Sized {

    fn apply_or<R, Y: FnOnce(Self) -> R, N: FnOnce(Self) -> R>(self, condition: bool, y: Y, n: N) -> R;

    fn apply_if<E, Y: FnOnce(Self) -> Result<Self, E>>(self, condition: bool, y: Y) -> Result<Self, E> {
        self.apply_or(condition, y, Ok)
    }
}

impl<T: Sized> FunctionalExt for T {
    fn apply_or<R, Y: FnOnce(Self) -> R, N: FnOnce(Self) -> R>(self, condition: bool, y: Y, n: N) -> R {
        match condition {
            true => y(self),
            false => n(self)
        }
    }
}

/*
pub trait OptionExt<T> {
    fn or_future(self, fut: impl Future<Output = Option<T>>) -> impl Future<Output = Option<T>>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_future(self, fut: impl Future<Output = Option<T>>) -> impl Future<Output = Option<T>> {
        async move {
            match self {
                Some(v) => Some(v),
                None => fut.await,
            }
        }
    }
}
 */


pub trait MutexExt {
    type Guard<'a>
    where
        Self: 'a;

    fn lock_no_poison(&self) -> Self::Guard<'_>;
}

impl<T> MutexExt for Mutex<T> {
    type Guard<'a>
        = MutexGuard<'a, T>
    where
        T: 'a;

    fn lock_no_poison(&self) -> Self::Guard<'_> {
        self.lock().unwrap_or_else(|err| err.into_inner())
    }
}
/*
pub trait ArcExt<T> {
    fn try_new_cyclic<F, E>(data_fn: F) -> std::result::Result<Arc<T>, E>
        where
            F: FnOnce(&Weak<T>) -> std::result::Result<T, E>;
}

impl<T> ArcExt<T> for Arc<T> {
    fn try_new_cyclic<F, E>(data_fn: F) -> std::result::Result<Arc<T>, E>
        where F: FnOnce(&Weak<T>) -> std::result::Result<T, E> {

        // hopefully this is safe
        let mut error: std::result::Result<(), E> = Ok(());
        let arc = Arc::<MaybeUninit<T>>::new_cyclic(|inner| {
             match data_fn(unsafe { std::mem::transmute(inner) }) {
                 Ok(r) => MaybeUninit::new(r),
                 Err(e) => {
                     error = Err(e);
                     MaybeUninit::uninit()
                 }
             }
        });
        error.map(|_| {
            let md_self = Arc::into_raw(arc);
            unsafe { Arc::<T>::from_raw(md_self.cast()) }
        })
        //Ok(Arc::new_cyclic(|i| data_fn(i).unwrap_or_else(|_| panic!("Closure panic"))))
    }
}

 */
