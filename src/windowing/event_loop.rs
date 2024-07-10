use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

use futures_lite::pin;
use windows::core::Error;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_ABANDONED_0, WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::{
    CreateEventExW, ResetEvent, SetEvent, CREATE_EVENT_INITIAL_SET, CREATE_EVENT_MANUAL_RESET, EVENT_MODIFY_STATE, INFINITE,
    SYNCHRONIZATION_SYNCHRONIZE
};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, MsgWaitForMultipleObjects, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, QS_ALLINPUT
};

use crate::runtime::process_timers_for_current_thread;
use crate::Result;

struct LoopWaker {
    event: HANDLE,
    awake: AtomicBool,
    notified: AtomicBool
}

impl LoopWaker {
    pub fn new() -> Result<Self> {
        let handle = unsafe {
            CreateEventExW(
                None,
                None,
                CREATE_EVENT_MANUAL_RESET | CREATE_EVENT_INITIAL_SET,
                (EVENT_MODIFY_STATE | SYNCHRONIZATION_SYNCHRONIZE).0
            )?
        };

        Ok(Self {
            event: handle,
            awake: AtomicBool::new(false),
            notified: AtomicBool::new(true)
        })
    }

    fn notify(&self) {
        if self.notified.swap(true, Ordering::SeqCst) || self.awake.load(Ordering::SeqCst) {
            return;
        }
        unsafe {
            SetEvent(self.event).unwrap_or_else(|err| log::warn!("Failed to signal event: {}", err));
        }
    }

    fn reset(&self) {
        unsafe {
            ResetEvent(self.event).unwrap_or_else(|err| log::warn!("Failed to reset event: {}", err));
        }
    }

    fn handle(&self) -> HANDLE {
        self.event
    }
}

impl Drop for LoopWaker {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.event).unwrap_or_else(|err| log::warn!("Failed to close handle: {}", err));
        }
    }
}

impl Wake for LoopWaker {
    fn wake(self: Arc<Self>) {
        self.notify();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.notify();
    }
}

pub fn event_loop<F: Future<Output = Result<()>>>(fut: F) -> Result<()> {
    pin!(fut);
    let notifier = Arc::new(LoopWaker::new()?);
    let waker = Waker::from(notifier.clone());

    loop {
        notifier.awake.store(true, Ordering::SeqCst);
        let mut next_timer;
        while {
            let mut cont = !pump_events();
            next_timer = process_timers_for_current_thread();
            if notifier.notified.swap(false, Ordering::SeqCst) {
                let mut cx = Context::from_waker(&waker);
                match fut.as_mut().poll(&mut cx) {
                    Poll::Ready(result) => return result,
                    Poll::Pending => {
                        cont = true;
                    } //pump_events()
                }
            }
            cont
        } {}

        notifier.reset();
        notifier.awake.store(false, Ordering::SeqCst);

        match wait_for(&[notifier.handle()], next_timer)? {
            WaitResult::Handle(_) => {}
            WaitResult::Message => {}
            WaitResult::Timeout => {}
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WaitResult {
    Handle(usize),
    Message,
    Timeout
}

fn wait_for(handles: &[HANDLE], timeout: Option<Duration>) -> windows::core::Result<WaitResult> {
    const SUCCESS: u32 = WAIT_OBJECT_0.0;
    const ABANDONED: u32 = WAIT_ABANDONED_0.0;
    let timeout = timeout
        .and_then(|d| d.as_millis().try_into().ok())
        .unwrap_or(INFINITE);
    let result = unsafe { MsgWaitForMultipleObjects(Some(handles), false, timeout, QS_ALLINPUT) };
    match result {
        r if (SUCCESS..SUCCESS + handles.len() as u32).contains(&r.0) => Ok(WaitResult::Handle((r.0 - SUCCESS) as usize)),
        r if r.0 == SUCCESS + handles.len() as u32 => Ok(WaitResult::Message),
        r if (ABANDONED..ABANDONED + handles.len() as u32).contains(&r.0) => panic!("One of the handles was abandoned"),
        WAIT_TIMEOUT => Ok(WaitResult::Timeout),
        WAIT_FAILED => Err(Error::from_win32()),
        _ => unreachable!()
    }
}

fn pump_events() -> bool {
    let mut limit = 10;
    let mut message = MSG::default();
    unsafe {
        while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
            limit -= 1;
            if limit <= 0 {
                return false;
            }
        }
    }
    true
}
