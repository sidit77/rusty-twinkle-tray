use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use windows::Win32::Devices::Display::*;
use windows::Win32::Foundation::{BOOL, HANDLE};
use windows::Win32::Graphics::Gdi::HMONITOR;

use crate::monitors::gdi::find_all_gdi_monitors;
use crate::monitors::paths::{find_all_paths, get_gdi_name, get_name_and_path};
use crate::utils::error::{Result, WinOptionExt};
use crate::utils::string::WStr;
use crate::win_assert;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MonitorPath(Arc<Path>);

impl MonitorPath {
    pub fn as_str(&self) -> &str {
        self.0.to_str().expect("MonitorPath is not valid UTF-8")
    }
}

impl Debug for MonitorPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<const N: usize> From<WStr<N>> for MonitorPath {
    fn from(value: WStr<N>) -> Self {
        Self(Arc::from(PathBuf::from(value)))
    }
}

impl From<&str> for MonitorPath {
    fn from(value: &str) -> Self {
        Self(Arc::from(PathBuf::from(value)))
    }
}

#[derive(Debug, Clone)]
pub struct Monitor {
    name: String,
    path: MonitorPath,
    hmonitor: HMONITOR
}

impl Monitor {
    pub fn find_all() -> Result<Vec<Monitor>> {
        let monitors = find_all_gdi_monitors()?;

        find_all_paths()?
            .into_iter()
            .map(|display| {
                let (name, path) = get_name_and_path(&display)?;
                let gdi = get_gdi_name(&display)?;
                let hmonitor = monitors.iter().find(|(n, _)| n == &gdi).some()?.1;
                Ok(Monitor { name, path, hmonitor })
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &MonitorPath {
        &self.path
    }

    pub fn open(&self) -> Result<MonitorConnection> {
        MonitorConnection::open(self.hmonitor)
    }
}

pub struct MonitorConnection {
    handle: HANDLE
}

impl MonitorConnection {
    fn open(monitor: HMONITOR) -> Result<Self> {
        win_assert!({
            let mut n = 0;
            unsafe {
                GetNumberOfPhysicalMonitorsFromHMONITOR(monitor, &mut n)?;
            };
            n == 1
        });
        let mut physical_monitor = PHYSICAL_MONITOR::default();
        unsafe { GetPhysicalMonitorsFromHMONITOR(monitor, std::slice::from_mut(&mut physical_monitor))? };

        Ok(Self {
            handle: physical_monitor.hPhysicalMonitor
        })
    }

    /*
    Seems to be broken
    pub fn get_capabilities(&self) -> Result<MonitorCapabilities> {
        let mut caps = 0;
        let mut temps = 0;
        dbg!(unsafe {  GetMonitorCapabilities(self.handle, &mut caps, &mut temps)});
        dbg!(unsafe { GetLastError()});
        Ok(MonitorCapabilities::from_bits_truncate(dbg!(caps)))
    }
     */

    pub fn get_brightness(&self) -> Result<(u32, RangeInclusive<u32>)> {
        let mut min = 0;
        let mut cur = 0;
        let mut max = 0;
        unsafe { BOOL(GetMonitorBrightness(self.handle, &mut min, &mut cur, &mut max)).ok()? };
        Ok((cur, min..=max))
    }

    pub fn set_brightness(&self, new: u32) -> Result<()> {
        unsafe { BOOL(SetMonitorBrightness(self.handle, new)).ok()? };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::error::Result;

    #[test]
    fn caps() -> Result<()> {
        for monitor in Monitor::find_all()? {
            println!("{}", monitor.name());
            let conn = monitor.open()?;
            let (current, range) = conn.get_brightness()?;
            println!("\tbrightness: {} / {}-{}", current, range.start(), range.end());
            conn.set_brightness(*range.start())?;
            //println!("\tcaps: {:?}", conn.get_capabilities()?);
        }
        Ok(())
    }
}

impl Drop for MonitorConnection {
    fn drop(&mut self) {
        unsafe { DestroyPhysicalMonitor(self.handle).unwrap_or_else(|err| log::warn!("Failed to release physical monitor: {err}")) }
    }
}

/*
bitflags! {

    #[derive(Debug, Copy, Clone)]
    pub struct MonitorCapabilities: u32 {
        const BRIGHTNESS = MC_CAPS_BRIGHTNESS;
        const COLOR_TEMPERATURE = MC_CAPS_COLOR_TEMPERATURE;
        const CONTRAST = MC_CAPS_CONTRAST;
        const DEGAUSS = MC_CAPS_DEGAUSS;
        const DISPLAY_AREA_POSITION = MC_CAPS_DISPLAY_AREA_POSITION;
        const DISPLAY_AREA_SIZE = MC_CAPS_DISPLAY_AREA_SIZE;
        const MONITOR_TECHNOLOGY_TYPE = MC_CAPS_MONITOR_TECHNOLOGY_TYPE;
        const RED_GREEN_BLUE_DRIVE = MC_CAPS_RED_GREEN_BLUE_DRIVE;
        const RED_GREEN_BLUE_GAIN = MC_CAPS_RED_GREEN_BLUE_GAIN;
        const RESTORE_FACTORY_COLOR_DEFAULTS = MC_CAPS_RESTORE_FACTORY_COLOR_DEFAULTS;
        const RESTORE_FACTORY_DEFAULTS = MC_CAPS_RESTORE_FACTORY_DEFAULTS;
        const RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS = MC_RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS;
    }
}
 */

type GdiName = WStr<32>;
mod paths {
    use std::mem::size_of;

    use windows::Win32::Devices::Display::{
        DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig, DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
        DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS
    };
    use windows::Win32::Foundation::WIN32_ERROR;

    use super::{GdiName, MonitorPath, WStr};
    use crate::utils::error::Result;

    pub fn find_all_paths() -> Result<Vec<DISPLAYCONFIG_PATH_INFO>> {
        unsafe {
            let mut path_count = 0;
            let mut mode_count = 0;
            GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count)?;

            let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); path_count as usize];
            let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); mode_count as usize];

            QueryDisplayConfig(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_count,
                paths.as_mut_ptr(),
                &mut mode_count,
                modes.as_mut_ptr(),
                None
            )?;

            Ok(paths)
        }
    }

    pub(super) fn get_gdi_name(path: &DISPLAYCONFIG_PATH_INFO) -> Result<GdiName> {
        let mut source_name = DISPLAYCONFIG_SOURCE_DEVICE_NAME {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
                size: size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32,
                adapterId: path.sourceInfo.adapterId,
                id: path.sourceInfo.id
            },
            ..Default::default()
        };

        unsafe { WIN32_ERROR(DisplayConfigGetDeviceInfo(&mut source_name.header) as u32).ok()? };

        Ok(source_name.viewGdiDeviceName.into())
    }

    pub(super) fn get_name_and_path(path: &DISPLAYCONFIG_PATH_INFO) -> Result<(String, MonitorPath)> {
        let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                size: size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
                adapterId: path.targetInfo.adapterId,
                id: path.targetInfo.id
            },
            ..Default::default()
        };

        unsafe { WIN32_ERROR(DisplayConfigGetDeviceInfo(&mut target_name.header) as u32).ok()? };

        let path = WStr::from(target_name.monitorDevicePath).into();
        let name = WStr::from(target_name.monitorFriendlyDeviceName).to_string_lossy();
        Ok((name, path))
    }
}

mod gdi {
    use std::mem::size_of;

    use windows::Win32::Foundation::{BOOL, LPARAM, RECT, TRUE};
    use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW};

    use super::GdiName;
    use crate::utils::error::Result;

    fn find_all_hmonitors() -> Result<Vec<HMONITOR>> {
        unsafe {
            let result = Box::into_raw(Box::default());
            unsafe extern "system" fn enum_func(hm: HMONITOR, _: HDC, _: *mut RECT, result: LPARAM) -> BOOL {
                let result = &mut *(result.0 as *mut Vec<HMONITOR>);
                result.push(hm);
                TRUE
            }
            EnumDisplayMonitors(None, None, Some(enum_func), LPARAM(result as _)).ok()?;
            Ok(*Box::from_raw(result))
        }
    }

    fn get_gdi_name(hm: HMONITOR) -> Result<GdiName> {
        let mut mi = MONITORINFOEXW {
            monitorInfo: MONITORINFO {
                cbSize: size_of::<MONITORINFOEXW>() as u32,
                ..Default::default()
            },
            ..Default::default()
        };
        unsafe { GetMonitorInfoW(hm, &mut mi as *mut _ as _).ok()? };
        Ok(mi.szDevice.into())
    }

    pub(super) fn find_all_gdi_monitors() -> Result<Vec<(GdiName, HMONITOR)>> {
        find_all_hmonitors()?
            .into_iter()
            .map(|hm| get_gdi_name(hm).map(|name| (name, hm)))
            .collect()
    }
}
