use std::mem::size_of;
use windows::core::{PCWSTR, w};
use windows::UI::Color;
use windows::Win32::System::Registry::{HKEY_CURRENT_USER, RegGetValueW, RRF_RT_REG_DWORD, RRF_SUBKEY_WOW6464KEY};

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
            Theme::Dark => w!("ImmersiveApplicationBackgroundDarkTheme"),
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
        let d = self.is_light()
            .then_some(255)
            .unwrap_or(0);
        Color {
            R: lerp(d, self.r, self.opacity),
            G: lerp(d, self.g, self.opacity),
            B: lerp(d, self.b, self.opacity),
            A: 255,
        }
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