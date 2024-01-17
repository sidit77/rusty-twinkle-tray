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

/*
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LocalTime {
    hours: u16,
    minutes: u16
}

impl LocalTime {
    pub fn current() -> Self {
        let time = unsafe { GetLocalTime() };
        Self {
            hours: time.wHour,
            minutes: time.wMinute,
        }
    }
}

impl Display for LocalTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl FromStr for LocalTime {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (h, m) = s
            .split_once(':')
            .ok_or("Missing separator (\":\")")?;
        Ok(Self {
            hours: h
                .parse()
                .map_err(|_| "Invalid hour string")
                .and_then(|h| (h < 24)
                    .then_some(h)
                    .ok_or("Hours can't be bigger than 23"))?,
            minutes: m
                .parse()
                .map_err(|_| "Invalid minutes string")
                .and_then(|m| (m < 60)
                    .then_some(m)
                    .ok_or("Minutes can't be bigger than 59"))?
        })
    }
}

impl Serialize for LocalTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for LocalTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: Deserializer<'de> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}
 */