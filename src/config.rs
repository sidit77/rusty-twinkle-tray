use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use log::{debug, warn};
use windows::core::imp::CoTaskMemFree;
use windows::Win32::UI::Shell::{FOLDERID_RoamingAppData, SHGetKnownFolderPath, KF_FLAG_DEFAULT};

use crate::monitors::MonitorPath;
use crate::Result;
use crate::windowing::hotkey::{KeyCombination, Modifier, VirtualKey};

#[derive(Debug)]
pub struct Config {
    pub dirty: bool,
    pub restore_from_config: bool,
    pub use_prioritized_autostart: bool,
    pub icon_scoll_enabled: bool,
    pub icon_scroll_step_size: f32,
    pub hotkeys_enabled: bool,
    pub hotkey_step_size: f32,
    pub brightness_increase_hotkey: KeyCombination,
    pub brightness_decrease_hotkey: KeyCombination,
    pub monitors: BTreeMap<MonitorPath, MonitorSettings>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dirty: true,
            restore_from_config: true,
            use_prioritized_autostart: false,
            icon_scoll_enabled: false,
            icon_scroll_step_size: 5.0,
            hotkeys_enabled: false,
            hotkey_step_size: 10.0,
            brightness_increase_hotkey: KeyCombination::from(([Modifier::Alt], VirtualKey::F1)),
            brightness_decrease_hotkey: KeyCombination::from(([Modifier::Alt], VirtualKey::F2)),
            monitors: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct MonitorSettings {
    pub saved_brightness: Option<u32>,
    pub custom_name: Option<String>,
    pub position: Option<i32>
}

impl Config {
    pub fn path() -> &'static Path {
        static PATH: OnceLock<PathBuf> = OnceLock::new();
        PATH.get_or_init(|| {
            unsafe { SHGetKnownFolderPath(&FOLDERID_RoamingAppData, KF_FLAG_DEFAULT, None) }
                .map(|ptr| unsafe {
                    let osstr = OsString::from_wide(ptr.as_wide());
                    CoTaskMemFree(ptr.as_ptr() as _);
                    osstr
                })
                .map(PathBuf::from)
                .unwrap_or_else(|err| {
                    log::warn!("Failed to retrieve AppData location: {}", err);
                    log::info!("Falling back to current directory...");
                    PathBuf::from(".")
                })
                .join("rusty-twinkle-tray.ini")
        })
    }

    pub fn restore() -> Result<Config> {
        let mut config = Self::default();
        if Self::path().exists() {
            config.load()?;
        }
        Ok(config)
    }

    pub fn save_if_dirty(&self) -> Result<()> {
        if self.dirty {
            self.save()?;
        }
        Ok(())
    }

    pub fn monitor(&mut self, path: &MonitorPath) -> &mut MonitorSettings {
        self.monitors.entry(path.clone()).or_default()
    }

    fn save(&self) -> Result<()> {
        let mut file = BufWriter::new(File::create(Self::path())?);

        write!(file, "[General]\r\n")?;
        write!(file, "RestoreFromConfig={}\r\n", self.restore_from_config)?;
        write!(file, "UsePrioritizedAutostart={}\r\n", self.use_prioritized_autostart)?;
        write!(file, "IconScrollEnabled={}\r\n", self.icon_scoll_enabled)?;
        write!(file, "IconScrollStepSize={}\r\n", self.icon_scroll_step_size)?;
        write!(file, "HotkeysEnabled={}\r\n", self.hotkeys_enabled)?;
        write!(file, "HotkeyStepSize={}\r\n", self.hotkey_step_size)?;
        write!(file, "BrightnessIncreaseHotkey={}\r\n", self.brightness_increase_hotkey.display(false))?;
        write!(file, "BrightnessDecreaseHotkey={}\r\n", self.brightness_decrease_hotkey.display(false))?;

        write!(file, "\r\n")?;

        for (path, settings) in &self.monitors {
            if settings.saved_brightness.is_none() && settings.custom_name.is_none() && settings.position.is_none() {
                continue;
            }
            write!(file, "[{}]\r\n", path.as_str())?;
            if let Some(brightness) = settings.saved_brightness {
                write!(file, "SavedBrightness={}\r\n", brightness)?;
            }
            if let Some(name) = &settings.custom_name {
                write!(file, "CustomName={}\r\n", name)?;
            }
            if let Some(order) = settings.position {
                write!(file, "Position={}\r\n", order)?;
            }
            write!(file, "\r\n")?;
        }

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        let file = BufReader::new(File::open(Self::path())?);
        let mut section = String::new();

        for (number, line) in file.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with('[') {
                if !line.ends_with(']') {
                    return Err(format!("Invalid section (Line: {number})").into());
                }
                section = line[1..line.len() - 1].to_string();
                continue;
            }

            if section.is_empty() {
                warn!("Line \"{line}\" (#{number}) is not in a section, ignoring it");
                continue;
            }

            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("Line \"{line}\" (#{number}) is not a key value pair"))?;
            
            match section.as_str() {
                "General" => match key {
                    "RestoreFromConfig" => self.restore_from_config = value.parse()?,
                    "UsePrioritizedAutostart" => self.use_prioritized_autostart = value.parse()?,
                    "IconScrollEnabled" => self.icon_scoll_enabled = value.parse()?,
                    "IconScrollStepSize" => self.icon_scroll_step_size = value.parse()?,
                    "HotkeysEnabled" => self.hotkeys_enabled = value.parse()?,
                    "HotkeyStepSize" => self.hotkey_step_size = value.parse()?,
                    "BrightnessIncreaseHotkey" => self.brightness_increase_hotkey = value.parse()?,
                    "BrightnessDecreaseHotkey" => self.brightness_decrease_hotkey = value.parse()?,
                    _ => debug!("Ignoring unknown key in section {}: {}={}", section, key, value)
                },
                path if path.starts_with("\\\\?\\DISPLAY") => {
                    let path = MonitorPath::from(path);
                    let settings = self.monitors.entry(path).or_default();
                    match key {
                        "SavedBrightness" => {
                            settings.saved_brightness = Some(value.parse()?);
                        }
                        "CustomName" => {
                            settings.custom_name = Some(value.to_string());
                        }
                        "Position" => {
                            settings.position = Some(value.parse()?);
                        }
                        _ => debug!("Ignoring unknown key in section {}: {}={}", section, key, value)
                    }
                }
                "" => debug!("Ignoring key without section: {}={}", key, value),
                _ => debug!("Ignoring key in unknown section {}: {}={}", section, key, value)
            }
        }

        Ok(())
    }
}

pub mod autostart {
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use log::warn;
    use registry::AutoStartRegKey;
    use windows::core::{w, PCWSTR};
    use windows::Win32::System::Registry::{KEY_READ, KEY_SET_VALUE};

    static EXE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
        std::env::current_exe()
            .and_then(dunce::canonicalize)
            .expect("Failed to get exe path")
    });

    // Programs seems to start in alphabetical order. So we prefix the name with an underscore.
    const PROGRAM_KEY: PCWSTR = w!("_RustyTwinkleTray");

    pub fn is_enabled(user: bool) -> bool {
        AutoStartRegKey::new(user, KEY_READ)
            .and_then(|reg| reg.read_path(PROGRAM_KEY))
            .map_err(|e| warn!("Failed to read registry: {e}"))
            .ok()
            .flatten()
            .map(|path| EXE_PATH.eq(&path))
            .unwrap_or(false)
    }

    pub fn set_enabled(user: bool, enabled: bool) -> crate::Result<()> {
        let reg = AutoStartRegKey::new(user, KEY_READ | KEY_SET_VALUE)?;
        match enabled {
            true => reg.write_path(PROGRAM_KEY, &EXE_PATH)?,
            false => reg.delete(PROGRAM_KEY)?
        }
        Ok(())
    }

    mod registry {
        use std::ffi::OsString;
        use std::mem::zeroed;
        use std::os::windows::ffi::{OsStrExt, OsStringExt};
        use std::path::{Path, PathBuf};
        use std::slice;

        use log::warn;
        use windows::core::{w, HRESULT, PCWSTR};
        use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA};
        use windows::Win32::System::Registry::{
            RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
            REG_EXPAND_SZ, REG_SAM_FLAGS, REG_SZ, REG_VALUE_TYPE
        };

        use crate::Result;

        pub struct AutoStartRegKey {
            handle: HKEY
        }

        impl AutoStartRegKey {
            pub fn new(user: bool, permissions: REG_SAM_FLAGS) -> Result<Self> {
                let handle = unsafe {
                    let mut handle = zeroed();
                    RegOpenKeyExW(
                        match user {
                            true => HKEY_CURRENT_USER,
                            false => HKEY_LOCAL_MACHINE
                        },
                        w!(r#"Software\Microsoft\Windows\CurrentVersion\Run"#),
                        0,
                        permissions,
                        &mut handle
                    )?;
                    handle
                };
                Ok(Self { handle })
            }

            pub fn read_path(&self, key: PCWSTR) -> Result<Option<PathBuf>> {
                let mut buffer = vec![0u16; 256];
                loop {
                    let mut size = buffer.len() as u32 * 2;
                    let mut ty = REG_VALUE_TYPE::default();
                    match unsafe { RegQueryValueExW(self.handle, key, None, Some(&mut ty), Some(buffer.as_mut_ptr() as _), Some(&mut size)) } {
                        Ok(()) => {
                            if !matches!(ty, REG_SZ | REG_EXPAND_SZ) {
                                return Err("Invalid registry item type".into());
                            }
                            let end = buffer.iter().take_while(|i| **i != 0).count();
                            return Ok(Some(PathBuf::from(OsString::from_wide(&buffer[..end]))));
                        }
                        Err(e) if e.code() == HRESULT::from_win32(ERROR_MORE_DATA.0) => buffer.resize(size as usize / 2, 0),
                        Err(e) if e.code() == HRESULT::from_win32(ERROR_FILE_NOT_FOUND.0) => return Ok(None),
                        Err(e) => return Err(e.into())
                    }
                }
            }

            pub fn delete(&self, key: PCWSTR) -> Result<()> {
                unsafe { RegDeleteValueW(self.handle, key)? };
                Ok(())
            }

            pub fn write_path(&self, key: PCWSTR, path: &Path) -> Result<()> {
                let path = path
                    .as_os_str()
                    .encode_wide()
                    .chain(Some(0))
                    .collect::<Vec<_>>();
                let path = unsafe { slice::from_raw_parts(path.as_ptr() as _, path.len() * 2) };
                unsafe { RegSetValueExW(self.handle, key, 0, REG_SZ, Some(path))? };
                Ok(())
            }
        }

        impl Drop for AutoStartRegKey {
            fn drop(&mut self) {
                unsafe {
                    RegCloseKey(self.handle).unwrap_or_else(|e| warn!("Failed to close registry key: {e}"));
                }
            }
        }
    }
}
