use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::File;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use ron::de::from_reader;
use ron::ser::{PrettyConfig, to_writer_pretty};
use serde::{Deserialize, Serialize};
use windows::core::imp::CoTaskMemFree;
use windows::Win32::UI::Shell::{FOLDERID_RoamingAppData, KF_FLAG_DEFAULT, SHGetKnownFolderPath};
use crate::monitors::MonitorPath;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub dirty: bool,
    pub restore_from_config: bool,
    pub monitors: BTreeMap<MonitorPath, MonitorSettings>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dirty: true,
            restore_from_config: true,
            monitors: Default::default(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MonitorSettings {
    pub saved_brightness: Option<u32>
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
                .join("rusty-twinkle-tray.ron")
        })
    }

    pub fn load() -> Result<Config> {
        Ok(match Self::path().exists() {
            true => {
                let file = File::open(Self::path())?;
                from_reader(file)
                    .map_err(|err| err.code)?
            }
            false => Config::default()
        })
    }

    pub fn save_if_dirty(&self) -> Result<()> {
        if self.dirty {
            let file = File::create(Self::path())?;
            to_writer_pretty(file, self, PrettyConfig::new())?;
        }
        Ok(())
    }

}