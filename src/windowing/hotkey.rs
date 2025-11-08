use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicI32, Ordering};
use std::task::{Context, Poll, Waker};
use futures_lite::Stream;
use log::{trace, warn};
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN, VIRTUAL_KEY};
use crate::Result;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Modifier {
    Shift = MOD_SHIFT.0 as u8,
    Ctrl = MOD_CONTROL.0 as u8,
    Alt = MOD_ALT.0 as u8,
    Win = MOD_WIN.0 as u8,
}



#[derive(Debug)]
pub struct HotKey {
    id: i32,
    _unsend: PhantomData<*const ()>,
}

impl HotKey {
    pub fn register(modifiers: HOT_KEY_MODIFIERS, key: VIRTUAL_KEY) -> Result<Self> {
        static NEXT_ID: AtomicI32 = AtomicI32::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        unsafe {
            RegisterHotKey(None, id, modifiers, key.0 as u32)?;
        }
        Ok(Self {
            id,
            _unsend: PhantomData
        })
    }
}

impl Drop for HotKey {
    fn drop(&mut self) {
        LOCAL_STATE.with(|hotkeys| {
            hotkeys.borrow_mut().remove(&self.id);
        });
        unsafe {
            UnregisterHotKey(None, self.id)
                .unwrap_or_else(|e| warn!("Failed to unregister hotkey: {:?}", e));
        }
    }
}

impl Stream for HotKey {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        LOCAL_STATE.with(|hotkeys| {
            let mut hotkeys = hotkeys.borrow_mut();
            let (triggered, waker) = hotkeys.entry(self.id).or_default();
            if *triggered > 0 {
                *triggered -= 1;
                Poll::Ready(Some(()))
            } else {
                *waker = Some(cx.waker().clone());
                Poll::Pending
            }
        })
    }
}

thread_local! { static LOCAL_STATE: RefCell<HashMap<i32, (u32, Option<Waker>)>> = RefCell::new(HashMap::new()); }

pub fn process_hotkey_for_current_thread(id: i32) {
    trace!("Processing hotkey (id: {id})");
    LOCAL_STATE.with(|hotkeys| {
        let mut hotkeys = hotkeys.borrow_mut();
        let (triggered, waker) = hotkeys.entry(id).or_default();
        *triggered += 1;
        if let Some(waker) = waker {
            waker.wake_by_ref();
        }
    });
}