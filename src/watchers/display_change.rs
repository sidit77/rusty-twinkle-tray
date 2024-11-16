use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{WM_DESTROY, WM_DISPLAYCHANGE};
use crate::Result;

pub struct DisplayChangeEventRegistration;

impl DisplayChangeEventRegistration {
    pub fn register<F: FnMut()>(hwnd: HWND, callback: F) -> Result<Self> {
        unsafe {
            SetWindowSubclass(
                hwnd,
                Some(Self::wnd_proc::<F>),
                super::DISPLAY_CHANGE_SUBCLASS_ID,
                Box::into_raw(Box::new(callback)) as _
            ).ok()?;
        }
        Ok(Self)
    }

    unsafe extern "system" fn wnd_proc<F: FnMut()>(
        hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, _id: usize, subclass_input_ptr: usize
    ) -> LRESULT {
        let callback_ptr = subclass_input_ptr as *mut F;
        match msg {
            WM_DESTROY => {
                drop(Box::from_raw(callback_ptr));
            }
            WM_DISPLAYCHANGE => {
                let callback = &mut *callback_ptr;
                callback();
            }
            _ => {}
        }
        DefSubclassProc(hwnd, msg, wparam, lparam)
    }
}
