use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND, WAIT_FAILED};
use windows::Win32::System::Threading::{WaitForSingleObject, INFINITE};
use windows::Win32::UI::Shell::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SEE_MASK_NO_CONSOLE, SHELLEXECUTEINFOW};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use crate::Result;

pub fn relaunch_as_elevated(parent: HWND, cmd: &str) -> Result<()> {
    let exe_path = std::env::current_exe()
        .expect("Failed to get current exe")
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let args = cmd
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<_>>();

    let handle = unsafe {
        let mut options = SHELLEXECUTEINFOW{
            cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
            fMask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NO_CONSOLE,
            hwnd: parent,
            lpVerb: w!("runas"),
            lpFile: PCWSTR(exe_path.as_ptr()),
            lpParameters: PCWSTR(args.as_ptr()),
            lpDirectory: PCWSTR::null(),
            nShow: SW_SHOWNORMAL.0,
            hInstApp: Default::default(),
            lpIDList: null_mut(),
            lpClass: PCWSTR::null(),
            hkeyClass: Default::default(),
            dwHotKey: 0,
            Anonymous: Default::default(),
            hProcess: Default::default(),
        };
        ShellExecuteExW(&mut options)?;
        ProcessHandle(options.hProcess)
    };

    handle.wait()?;

    Ok(())
}

struct ProcessHandle(HANDLE);

impl ProcessHandle {
    pub fn wait(&self) -> Result<()> {
        unsafe {
            if WaitForSingleObject(self.0, INFINITE) == WAIT_FAILED {
                return Err(windows::core::Error::from_win32().into());
            }
        }
        Ok(())
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0)
                .unwrap_or_else(|e| log::warn!("Failed to close process handle: {e}"));
        };
    }
}