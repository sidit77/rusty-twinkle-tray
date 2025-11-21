use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use log::{warn};
use windows::core::{GUID};
use windows::Win32::Foundation::{BOOL, FALSE, HINSTANCE, HWND, LPARAM, LRESULT, RECT, TRUE, WPARAM};
use windows::Win32::System::SystemInformation::GetTickCount64;
use windows::Win32::UI::Shell::{Shell_NotifyIconGetRect, NOTIFYICONIDENTIFIER};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, EnumWindows, GetClassNameA, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, HHOOK, MSLLHOOKSTRUCT, WHEEL_DELTA, WH_MOUSE_LL, WM_MOUSEWHEEL};
use crate::Result;

struct CallbackContext {
    tray_id: NOTIFYICONIDENTIFIER,
    tray_rect: RECT,
    last_rect_query: u64,
    callback: Box<dyn FnMut(f32)>
}

impl CallbackContext {

    fn new<F: FnMut(f32)+ 'static>(id: NOTIFYICONIDENTIFIER, callback: F) -> Self {
        Self {
            tray_id: id,
            tray_rect: Default::default(),
            last_rect_query: 0,
            callback: Box::new(callback),
        }
    }

    fn cached_tray_rect(&mut self) -> RECT {
        let current_time = unsafe { GetTickCount64() };
        if current_time.saturating_sub(self.last_rect_query) > 1000 {
            self.tray_rect = unsafe { Shell_NotifyIconGetRect(&self.tray_id) }
                //.inspect(|rect| trace!("Found tray rect: {:?}", rect))
                .map_err(|err| warn!("Failed to query tray rect: {}", err))
                .unwrap_or(self.tray_rect);
            self.last_rect_query = current_time;
        }
        self.tray_rect
    }
}

thread_local! { static CONTEXT: Cell<Option<CallbackContext>> = Cell::new(None) }

pub struct TrayIconScrollCallback {
    hook: HHOOK,
    _unsent: PhantomData<*mut ()>
}

impl Debug for TrayIconScrollCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrayIconScrollCallback").finish_non_exhaustive()
    }
}

impl Drop for TrayIconScrollCallback {
    fn drop(&mut self) {
        CONTEXT.set(None);
        unsafe {
            UnhookWindowsHookEx(self.hook)
                .unwrap_or_else(|err| warn!("Failed to remove mouse hook: {err}"));
        }
    }
}

impl TrayIconScrollCallback {
    pub fn new<F: FnMut(f32) + 'static>(callback: F) -> Result<Self> {
        let id = find_tray_icon_identifier()?;

        CONTEXT.set(Some(CallbackContext::new(id, callback)));
        let hook = unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), HINSTANCE::default(), 0)? };

        Ok(Self {
            hook,
            _unsent: PhantomData,
        })
    }

}

unsafe extern "system" fn low_level_mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let event_data = (lparam.0 as *const MSLLHOOKSTRUCT).read();
    if code == HC_ACTION as i32 {
        match wparam.0 as u32 {
            WM_MOUSEWHEEL => if let Some(mut context) = CONTEXT.take() {
                if in_rect(context.cached_tray_rect(), event_data.pt.x, event_data.pt.y) {
                    (context.callback)(parse_wheel_delta(&event_data));
                }
                CONTEXT.set(Some(context));
            }
            _ => {}
        }
    }
    CallNextHookEx(HHOOK::default(), code, wparam, lparam)
}

fn in_rect(rect: RECT, x: i32, y: i32) -> bool {
    x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
}

fn parse_wheel_delta(event_data: &MSLLHOOKSTRUCT) -> f32 {
    (event_data.mouseData >> 16) as i16 as f32 / WHEEL_DELTA as f32
}

// betrayer doesn't yet export a way to get the platform-specific primitives
// To work around this we manually find its hidden window
fn find_tray_icon_identifier() -> Result<NOTIFYICONIDENTIFIER> {
    unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        const TARGET_CLASS: &[u8] = b"tray_icon_window";
        let mut buf = [0u8; 32];
        let len = GetClassNameA(hwnd, &mut buf) as usize;
        if len != 0 && &buf[..len] == TARGET_CLASS {
            let param = lparam.0 as *mut Option<HWND>;
            param.write(Some(hwnd));
            FALSE
        } else {
            TRUE
        }
    }

    let mut target_window: Option<HWND> = None;
    unsafe { let _ = EnumWindows(Some(enum_window_proc), LPARAM(&mut target_window as *mut _ as _)); }

    Ok(NOTIFYICONIDENTIFIER {
        cbSize: size_of::<NOTIFYICONIDENTIFIER>() as u32,
        hWnd: target_window.ok_or("Failed to find tray icon parent window")?,
        uID: 1, //betrayer numbers tray icons in ascending order
        guidItem: GUID::zeroed(),
    })
}
