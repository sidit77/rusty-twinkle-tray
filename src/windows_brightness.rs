//! Two-way sync between the monitors and Windows' own brightness value
//! (the "Display brightness" setting of the active power plan).
//!
//! * Follow (Windows -> monitors): Windows changes this value when the brightness keys are pressed
//!   (both HID keys and keys handled by laptop firmware) or its brightness slider is moved.
//!   We get notified and move all monitors by the same amount.
//! * Mirror (monitors -> Windows): whenever the primary monitor changes for another reason
//!   (slider, scroll, hotkeys, restore), its level is written back to Windows, so that Windows' popup
//!   shows the real level and the next key press continues from there.
//!
//! All decisions happen on the main thread, which both receives Windows' notifications and owns the
//! sliders, so the known Windows value and the monitor levels are always compared at the same point in
//! time. Comparing them across threads lets a key press that is still being applied look like a
//! change that needs writing back, which pushes Windows back to the old level.
//!
//! Writing the value makes Windows notify us again. These echoes match the value we already
//! know Windows has, so they are ignored.

use std::ptr::null_mut;
use std::thread::spawn;

use log::{debug, trace, warn};
use loole::{unbounded, Sender};
use windows::core::GUID;
use windows::Win32::Foundation::{LocalFree, HLOCAL, WIN32_ERROR};
use windows::Win32::System::Power::{PowerGetActiveScheme, PowerSetActiveScheme, PowerWriteACValueIndex, PowerWriteDCValueIndex};
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::SystemServices::{GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS, GUID_VIDEO_SUBGROUP};

use crate::Result;

pub struct WindowsBrightness {
    /// The last brightness value we know Windows has
    known: Option<u32>,
    writer: Sender<u32>
}

impl WindowsBrightness {
    pub fn new() -> Self {
        // Applying a power scheme can take a moment, so writes happen on their own thread.
        // Only the newest pending value matters; older ones are skipped.
        let (writer, receiver) = unbounded::<u32>();
        spawn(move || {
            while let Ok(mut value) = receiver.recv() {
                while let Ok(newer) = receiver.try_recv() {
                    value = newer;
                }
                debug!("Writing brightness {} to Windows", value);
                write(value).unwrap_or_else(|err| warn!("Failed to write the Windows brightness: {err}"));
            }
        });
        Self { known: None, writer }
    }

    /// Called when Windows reports a new brightness value.
    /// Returns by how much the monitors should change, or `None` if nothing should happen.
    ///
    /// `initial` marks the notification Windows sends right after registering. It only reports the
    /// starting value and may be older than a value we already wrote, so it never moves the monitors.
    pub fn on_windows_changed(&mut self, value: u32, initial: bool) -> Option<i32> {
        if initial {
            if self.known.is_none() {
                debug!("Initial Windows brightness: {}", value);
                self.known = Some(value);
            }
            return None;
        }
        match self.known.replace(value) {
            None => None,
            Some(previous) if previous == value => {
                trace!("Ignoring Windows brightness notification for our own write ({})", value);
                None
            }
            Some(previous) => Some(value as i32 - previous as i32)
        }
    }

    /// Writes `level` into Windows' brightness setting, unless Windows already has it.
    pub fn mirror(&mut self, level: u32) {
        if self.known == Some(level) {
            trace!("Windows brightness already at {}", level);
            return;
        }
        debug!("Mirroring brightness {} to Windows (was {:?})", level, self.known);
        self.known = Some(level);
        let _ = self.writer.send(level);
    }
}

fn write(value: u32) -> Result<()> {
    unsafe {
        let mut scheme: *mut GUID = null_mut();
        PowerGetActiveScheme(HKEY::default(), &mut scheme)?;
        let result = write_to_scheme(scheme, value);
        let _ = LocalFree(HLOCAL(scheme as _));
        result
    }
}

unsafe fn write_to_scheme(scheme: *const GUID, value: u32) -> Result<()> {
    // The setting is stored separately for "plugged in" (AC) and "on battery" (DC)
    WIN32_ERROR(PowerWriteACValueIndex(
        HKEY::default(),
        scheme,
        Some(&GUID_VIDEO_SUBGROUP),
        Some(&GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS),
        value
    ))
    .ok()?;
    WIN32_ERROR(PowerWriteDCValueIndex(
        HKEY::default(),
        scheme,
        Some(&GUID_VIDEO_SUBGROUP),
        Some(&GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS),
        value
    ))
    .ok()?;
    // Written values only take effect once the scheme is applied again
    PowerSetActiveScheme(HKEY::default(), Some(scheme))?;
    Ok(())
}
