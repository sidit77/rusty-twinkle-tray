use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Write};
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::Win32::Graphics::Gdi::HMONITOR;
use crate::error::OptionExt;
use crate::monitors::gdi::find_all_gdi_monitors;
use crate::monitors::paths::{find_all_paths, get_gdi_name, get_name_and_path};

use crate::Result;

#[derive(Debug, Clone)]
pub struct Monitor {
    name: String,
    path: PathBuf,
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
                let hmonitor = monitors
                    .iter()
                    .find(|(n, _)| n == &gdi)
                    .some()?
                    .1;
                Ok(Monitor {
                    name,
                    path,
                    hmonitor,
                })
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }



}

#[derive(Copy, Clone, PartialEq, Eq)]
struct WStr<const N: usize>([u16; N]);

impl<const N: usize> From<[u16; N]> for WStr<N> {
    fn from(value: [u16; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Display for WStr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in char::decode_utf16(self.as_slice().iter().copied()) {
            f.write_char(c.unwrap_or(std::char::REPLACEMENT_CHARACTER))?
        }
        Ok(())
    }
}

impl<const N: usize> Debug for WStr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl<const N: usize> From<WStr<N>> for OsString {
    fn from(value: WStr<N>) -> Self {
        OsString::from_wide(value.as_slice())
    }
}

impl<const N: usize> From<WStr<N>> for PathBuf {
    fn from(value: WStr<N>) -> Self {
        PathBuf::from(OsString::from(value))
    }
}

impl<const N: usize> WStr<N> {
    pub fn as_slice(&self) -> &[u16] {
        let end = self.0
            .iter()
            .position(|c| *c == 0)
            .unwrap_or(self.0.len());
        &self.0[..end]
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

type GdiName = WStr<32>;
mod paths {
    use std::mem::size_of;
    use std::path::PathBuf;
    use windows::Win32::Devices::Display::{DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME, DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QDC_ONLY_ACTIVE_PATHS, QueryDisplayConfig};
    use windows::Win32::Foundation::WIN32_ERROR;
    use crate::error::Result;
    use super::{GdiName, WStr};

    pub fn find_all_paths() -> Result<Vec<DISPLAYCONFIG_PATH_INFO>> {
        unsafe {
            let mut path_count = 0;
            let mut mode_count = 0;
            GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count)?;

            let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); path_count as usize];
            let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); mode_count as usize];

            QueryDisplayConfig(QDC_ONLY_ACTIVE_PATHS, &mut path_count, paths.as_mut_ptr(), &mut mode_count, modes.as_mut_ptr(), None)?;

            Ok(paths)
        }
    }

    pub(super) fn get_gdi_name(path: &DISPLAYCONFIG_PATH_INFO) -> Result<GdiName> {
        let mut source_name = DISPLAYCONFIG_SOURCE_DEVICE_NAME {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
                size: size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32,
                adapterId: path.sourceInfo.adapterId,
                id: path.sourceInfo.id,
            },
            ..Default::default()
        };

        unsafe { WIN32_ERROR( DisplayConfigGetDeviceInfo(&mut source_name.header) as u32).ok()? };

        Ok(source_name.viewGdiDeviceName.into())
    }

    pub(super) fn get_name_and_path(path: &DISPLAYCONFIG_PATH_INFO) -> Result<(String, PathBuf)> {
        let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME  {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                size: size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
                adapterId: path.targetInfo.adapterId,
                id: path.targetInfo.id,
            },
            ..Default::default()
        };

        unsafe { WIN32_ERROR( DisplayConfigGetDeviceInfo(&mut target_name.header) as u32).ok()? };

        let path = WStr::from(target_name.monitorDevicePath).into();
        let name = WStr::from(target_name.monitorFriendlyDeviceName).to_string_lossy();
        Ok((name, path))
    }

}

mod gdi {
    use std::mem::size_of;
    use windows::Win32::Foundation::{BOOL, LPARAM, RECT, TRUE};
    use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW};
    use crate::error::Result;
    use super::GdiName;

    fn find_all_hmonitors() -> Result<Vec<HMONITOR>> {
        unsafe {
            let result = Box::into_raw(Box::new(Vec::new()));
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