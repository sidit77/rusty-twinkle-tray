use std::mem::size_of;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::Shell::{NIF_ICON, NIF_MESSAGE, NIM_ADD, NOTIFYICONDATAW, Shell_NotifyIconW};
use windows::Win32::UI::WindowsAndMessaging::{IDI_QUESTION, LoadIconW, WM_USER};
use crate::utils::error::Result;

const WM_USER_TRAY_ICON: u32 = WM_USER + 1024;

pub fn tray(instance: HMODULE, window: HWND) -> Result<()> {
    let icon = unsafe { LoadIconW(None, IDI_QUESTION)? };
    let notify_icon_data = NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        uFlags: NIF_MESSAGE | NIF_ICON/* | NIF_TIP*/,
        hWnd: window,
        uID: 1,
        hIcon: icon,
        /*szTip: */
        uCallbackMessage: WM_USER_TRAY_ICON,
        ..Default::default()
    };

    unsafe { Shell_NotifyIconW(NIM_ADD, &notify_icon_data).ok()? };
    Ok(())
}