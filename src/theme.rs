use std::mem::size_of;

use windows::core::{w, PCWSTR};
use windows::Win32::System::Registry::{RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WOW64_64KEY};
use windows::UI::Color;
use windows::UI::ViewManagement::{UIColorType, UISettings};
use windows_ext::UI::Xaml::ElementTheme;

use crate::Result;

pub struct ColorSet {
    pub tint: Color,
    pub fallback: Color,
    pub opacity: f64,
    pub theme: ElementTheme
}

impl ColorSet {
    pub fn dark() -> Self {
        Self {
            tint: Color { R: 0, G: 0, B: 0, A: 255 },
            fallback: Color { R: 0, G: 0, B: 0, A: 255 },
            opacity: 0.6,
            theme: ElementTheme::Dark
        }
    }

    pub fn light() -> Self {
        Self {
            tint: Color {
                R: 255,
                G: 255,
                B: 255,
                A: 255
            },
            fallback: Color {
                R: 255,
                G: 255,
                B: 255,
                A: 255
            },
            opacity: 0.7,
            theme: ElementTheme::Light
        }
    }

    pub fn accent(color: Color) -> Self {
        Self {
            tint: color,
            fallback: color,
            opacity: 0.7,
            theme: ElementTheme::Dark
        }
    }

    pub fn system(system_settings: &SystemSettings, ui_settings: &UISettings) -> Self {
        system_settings
            .is_accent_enabled()
            .map_err(|err| log::warn!("Could not detect accent state: {err}"))
            .ok()
            .and_then(|enabled| {
                enabled
                    .then_some(UIColorType::AccentDark1)
                    .and_then(|c| {
                        ui_settings
                            .GetColorValue(c)
                            .map_err(|err| log::warn!("Failed to query accent color: {err}"))
                            .ok()
                    })
                    .map(Self::accent)
            })
            .or_else(|| {
                system_settings
                    .is_system_theme_light()
                    .map_err(|err| log::warn!("Could not detect system theme state: {err}"))
                    .ok()
                    .and_then(|enabled| enabled.then_some(Self::light()))
            })
            .unwrap_or(Self::dark())
    }
}

/*
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Accent
}

impl Theme {

    pub fn system() -> Self {
        get_theme_setting(w!("ColorPrevalence"))
            .then_some(Self::Accent)
            .or_else(|| get_theme_setting(w!("SystemUsesLightTheme"))
                .then_some(Self::Light))
            .unwrap_or(Self::Dark)
    }

    pub fn opacity(self) -> f32 {
        match self {
            Theme::Light => 0.8,
            Theme::Dark => 0.7,
            Theme::Accent => 0.8
        }
    }

    fn color_name(self) -> PCWSTR {
        match self {
            Theme::Light => w!("ImmersiveLightChromeMedium"),
            Theme::Dark => w!("ImmersiveDarkChromeMedium"),
            Theme::Accent => w!("ImmersiveSystemAccentDark1")
        }
    }

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThemeColor {
    r: u8,
    g: u8,
    b: u8,
    opacity: f32
}

impl ThemeColor {

    pub fn tint_color(self) -> Color {
        Color { R: self.r, G: self.g, B: self.b, A: 255 }
    }

    pub fn opacity(self) -> f64 {
        self.opacity as f64
    }

    pub fn fallback_color(self) -> Color {
        //let d = self.is_light()
        //    .then_some(255)
        //    .unwrap_or(0);
        //Color {
        //    R: lerp(d, self.r, self.opacity),
        //    G: lerp(d, self.g, self.opacity),
        //    B: lerp(d, self.b, self.opacity),
        //    A: 255,
        //}
        Color { R: self.r, G: self.g, B: self.b, A: 255 }
    }

    pub fn is_light(self) -> bool {
        false
    }
}

impl From<Theme> for ThemeColor {
    fn from(value: Theme) -> Self {
        let color = unsafe {
            let color_set = GetImmersiveUserColorSetPreference(false, false);
            let color_type = GetImmersiveColorTypeFromName(value.color_name());
            GetImmersiveColorFromColorSetEx(color_set, color_type, false, 0)
                .to_ne_bytes()
        };
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
            opacity: value.opacity(),
        }
    }
}

#[link(name = "uxtheme", kind = "raw-dylib")]
extern "system" {
    #[link_ordinal(94)]
    fn GetImmersiveColorSetCount() -> u32;

    #[link_ordinal(95)]
    fn GetImmersiveColorFromColorSetEx(dwImmersiveColorSet: u32, dwImmersiveColorType: u32, bIgnoreHighContrast: bool, dwHighContrastCacheMode: u32) -> u32;

    #[link_ordinal(96)]
    fn GetImmersiveColorTypeFromName(name: PCWSTR) -> u32;

    #[link_ordinal(98)]
    fn GetImmersiveUserColorSetPreference(bForceCheckRegistry: bool, bSkipCheckOnFail: bool) -> u32;

    #[link_ordinal(100)]
    fn GetImmersiveColorNamedTypeByIndex(dwImmersiveColorType: u32) -> *const PCWSTR;
}

*/
pub struct SystemSettings {
    key: HKEY
}

impl SystemSettings {
    pub fn new() -> crate::Result<Self> {
        let mut key = HKEY::default();
        unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                w!(r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#),
                0,
                KEY_READ | KEY_WOW64_64KEY,
                &mut key
            )
        }?;

        Ok(Self { key })
    }

    /*
        pub fn add_change_callback<F>(self: &Arc<Self>, callback: F) -> Result<SystemSettingsChangedCallback>
            where F : FnMut(&SystemSettings) + Send + 'static
        {
            SystemSettingsChangedCallbackInner::new(self.clone(), Box::new(callback))
                .map(SystemSettingsChangedCallback)
        }
    */

    unsafe fn query_value(&self, name: PCWSTR) -> Result<bool> {
        let mut data: u32 = 0;
        unsafe {
            RegQueryValueExW(
                self.key,
                name,
                None,
                None,
                Some(&mut data as *const _ as _),
                Some(&mut (size_of::<u32>() as u32))
            )?;
        }
        Ok(data > 0)
    }

    pub fn is_accent_enabled(&self) -> Result<bool> {
        unsafe { self.query_value(w!("ColorPrevalence")) }
    }

    pub fn is_system_theme_light(&self) -> Result<bool> {
        unsafe { self.query_value(w!("SystemUsesLightTheme")) }
    }
}

impl Drop for SystemSettings {
    fn drop(&mut self) {
        unsafe {
            RegCloseKey(self.key).unwrap_or_else(|err| log::warn!("Failed to close registry key: {err}"));
        }
    }
}

/*
pub struct SystemSettingsChangedCallback(Arc<SystemSettingsChangedCallbackInner>);

impl SystemSettingsChangedCallback {
    pub fn detach(self) {
        std::mem::forget(self)
    }
}

struct SystemSettingsChangedCallbackInner {
    settings: Arc<SystemSettings>,
    callback: Mutex<Box<dyn FnMut(&SystemSettings) + Send>>,
    event: HANDLE,
    waiter: PTP_WAIT
}

impl SystemSettingsChangedCallbackInner {
    fn new(settings: Arc<SystemSettings>, callback: Box<dyn FnMut(&SystemSettings) + Send>) -> Result<Arc<Self>> {
        let r = Arc::try_new_cyclic::<_, TracedError>(move |weak| {
            let event = unsafe { CreateEventW(None, true, false, None)? };
            let waiter = unsafe { CreateThreadpoolWait(Some(Self::callback_func), Some(weak.clone().into_raw() as _), None)? };
            Ok(Self {
                settings,
                callback: Mutex::new(callback),
                event,
                waiter,
            })
        })?;
        r.register()?;
        Ok(r)
    }

    fn register(&self) -> Result<()> {
        unsafe {
            ResetEvent(self.event)?;
            RegNotifyChangeKeyValue(
                self.settings.key,
                false,
                REG_NOTIFY_CHANGE_LAST_SET | REG_NOTIFY_THREAD_AGNOSTIC,
                self.event,
                true
            )?;
            SetThreadpoolWait(self.waiter, self.event, None);
        }
        Ok(())
    }

    unsafe extern "system" fn callback_func(_: PTP_CALLBACK_INSTANCE, context: *mut c_void, _: PTP_WAIT, _: u32) {
        let context = ManuallyDrop::new(Weak::<Self>::from_raw(context as _));
        match context.upgrade() {
            Some(context) => {
                (context.callback.lock_no_poison())(&context.settings);
                context.register()
                    .unwrap_or_else(|err| log::warn!("Failed to reregister callback: {err}"));
            }
            None => log::error!("Weak ptr is gone :(")
        }
    }

}

impl Drop for SystemSettingsChangedCallbackInner {
    fn drop(&mut self) {
        unsafe {
            CloseThreadpoolWait(self.waiter);
            CloseHandle(self.event)
                .unwrap_or_else(|err| log::warn!("Failed to destroy event: {err}"));
        }
    }
}

pub fn get_theme_setting(name: PCWSTR) -> bool {
    unsafe {
        let mut data: u32 = 0;
        RegGetValueW(
            HKEY_CURRENT_USER,
            w!(r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#),
            name,
            RRF_RT_REG_DWORD | RRF_SUBKEY_WOW6464KEY,
            None,
            Some(&mut data as *const _ as _),
            Some(&mut (size_of::<u32>() as u32))
        ).unwrap();
        data > 0
    }
}

fn lerp(a: u8, b: u8, v: f32) -> u8 {
    ((a as f32) * (1.0 - v) + (b as f32) * v) as u8
}

 */
