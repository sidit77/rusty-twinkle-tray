use std::cell::RefCell;
use std::char::REPLACEMENT_CHARACTER;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::marker::PhantomData;
use std::num::{NonZeroU32, NonZeroUsize};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::atomic::{AtomicI32, Ordering};
use std::task::{Context, Poll, Waker};
use futures_lite::Stream;
use log::{trace, warn};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyNameTextW, MapVirtualKeyW, RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MAPVK_VK_TO_VSC, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN, VIRTUAL_KEY, VK_CONTROL, VK_ESCAPE, VK_F1, VK_F2, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_MENU, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SHIFT};
use crate::Result;

#[derive(Debug)]
pub struct HotKey {
    id: i32,
    _unsend: PhantomData<*const ()>,
}

impl HotKey {
    pub fn register(combo: impl Into<KeyCombination>) -> Result<Self> {
        static NEXT_ID: AtomicI32 = AtomicI32::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let KeyCombination { modifiers, key } = combo.into();
        unsafe {
            RegisterHotKey(None, id, modifiers.as_raw(), key.0)?;
        }
        Ok(Self {
            id,
            _unsend: PhantomData
        })
    }
}

impl Drop for HotKey {
    fn drop(&mut self) {
        LOCAL_STATE.with(|hotkeys| {
            hotkeys.borrow_mut().remove(&self.id);
        });
        unsafe {
            UnregisterHotKey(None, self.id)
                .unwrap_or_else(|e| warn!("Failed to unregister hotkey: {:?}", e));
        }
    }
}

impl Stream for HotKey {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        LOCAL_STATE.with(|hotkeys| {
            let mut hotkeys = hotkeys.borrow_mut();
            let (triggered, waker) = hotkeys.entry(self.id).or_default();
            if *triggered > 0 {
                *triggered -= 1;
                Poll::Ready(Some(()))
            } else {
                *waker = Some(cx.waker().clone());
                Poll::Pending
            }
        })
    }
}

thread_local! { static LOCAL_STATE: RefCell<HashMap<i32, (u32, Option<Waker>)>> = RefCell::new(HashMap::new()); }

pub fn process_hotkey_for_current_thread(id: i32) {
    trace!("Processing hotkey (id: {id})");
    LOCAL_STATE.with(|hotkeys| {
        let mut hotkeys = hotkeys.borrow_mut();
        let (triggered, waker) = hotkeys.entry(id).or_default();
        *triggered += 1;
        if let Some(waker) = waker {
            waker.wake_by_ref();
        }
    });
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyCombination {
    pub modifiers: ModifierSet,
    pub key: VirtualKey,
}

impl From<VirtualKey> for KeyCombination {
    fn from(value: VirtualKey) -> Self {
        Self {
            modifiers: ModifierSet::empty(),
            key: value,
        }
    }
}

impl From<VIRTUAL_KEY> for KeyCombination {
    fn from(value: VIRTUAL_KEY) -> Self {
        Self::from(VirtualKey::from(value))
    }
}

impl<const N: usize> From<([Modifier; N], VirtualKey)> for KeyCombination {
    fn from((modifiers, key): ([Modifier; N], VirtualKey)) -> Self {
        Self { modifiers: ModifierSet::from_iter(modifiers), key }
    }

}

impl<const N: usize> From<([Modifier; N], VIRTUAL_KEY)> for KeyCombination {
    fn from((modifiers, key): ([Modifier; N], VIRTUAL_KEY)) -> Self {
        Self::from((modifiers, VirtualKey::from(key)))
    }

}

impl KeyCombination {
    pub fn display(self, use_key_name: bool) -> impl Display {
        struct KeyCombinationDisplay(KeyCombination, bool);

        impl Display for KeyCombinationDisplay {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                for m in self.0.modifiers {
                    f.write_fmt(format_args!("{} + ", m.name()))?;
                }
                if self.1 {
                    f.write_fmt(format_args!("{}", self.0.key.name()))?;
                } else {
                    f.write_fmt(format_args!("0x{:X}", self.0.key.0))?;
                }
                Ok(())
            }
        }

        KeyCombinationDisplay(self, use_key_name)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseKeyCombinationError {
    UnknownModifier,
    MalformattedKeyCode
}

impl Display for ParseKeyCombinationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownModifier => f.write_str("Unknown modifier"),
            Self::MalformattedKeyCode => f.write_str("Malformatted key code")
        }
    }
}

impl std::error::Error for ParseKeyCombinationError {}

impl FromStr for KeyCombination {
    type Err = ParseKeyCombinationError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut tokens = s.rsplit('+');
        let keycode = tokens
            .next()
            .ok_or(ParseKeyCombinationError::MalformattedKeyCode)?
            .trim()
            .trim_start_matches("0x");
        let keycode = u32::from_str_radix(keycode, 16)
            .map_err(|_| ParseKeyCombinationError::MalformattedKeyCode)?;

        let modifiers: ModifierSet = tokens
            .map(str::trim)
            .map(|s| [Modifier::Alt, Modifier::Ctrl, Modifier::Shift, Modifier::Shift]
                .into_iter()
                .find(|&m| m.name().eq_ignore_ascii_case(s))
                .ok_or(ParseKeyCombinationError::UnknownModifier))
            .collect::<std::result::Result<_, _>>()?;

        Ok(Self {
            modifiers,
            key: VirtualKey(keycode),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtualKey(u32);

impl VirtualKey {

    pub const ESCAPE: VirtualKey = Self::from_vk(VK_ESCAPE);
    pub const F1: VirtualKey = Self::from_vk(VK_F1);
    pub const F2: VirtualKey = Self::from_vk(VK_F2);

    const fn from_vk(vk: VIRTUAL_KEY) -> Self { Self(vk.0 as u32) }
    const fn as_vk(self) -> VIRTUAL_KEY { VIRTUAL_KEY(self.0 as u16) }

    pub fn name(self) -> impl Display {
        struct KeyName(u32);

        impl Display for KeyName {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut buf = [0u16; 32];


                let success = NonZeroU32::new(unsafe { MapVirtualKeyW(self.0, MAPVK_VK_TO_VSC) })
                    .map(|sc| (sc.get() as i32) << 16)
                    .and_then(|sc| NonZeroUsize::new(unsafe { GetKeyNameTextW(sc, &mut buf) } as usize))
                    .is_some();

                if success {
                    std::char::decode_utf16(buf.into_iter().take_while(|&c| c != 0))
                        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
                        .try_for_each(|c| f.write_char(c))
                } else {
                    f.write_fmt(format_args!("0x{:X}", self.0))
                }
            }
        }

        KeyName(self.0)

    }

    pub fn is_modifier(self) -> bool {
        matches!(self.as_vk(),
            VK_LMENU | VK_RMENU | VK_MENU |
            VK_LSHIFT | VK_RSHIFT | VK_SHIFT |
            VK_LCONTROL | VK_RCONTROL | VK_CONTROL |
            VK_LWIN | VK_RWIN)
    }

}

impl From<VIRTUAL_KEY> for VirtualKey {
    fn from(value: VIRTUAL_KEY) -> Self {
        Self::from_vk(value)
    }
}

impl From<windows::System::VirtualKey> for VirtualKey {
    fn from(value: windows::System::VirtualKey) -> Self {
        Self(value.0 as u32)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Modifier {
    Shift = MOD_SHIFT.0 as u8,
    Ctrl = MOD_CONTROL.0 as u8,
    Alt = MOD_ALT.0 as u8,
    Win = MOD_WIN.0 as u8,
}

impl Modifier {
    const fn as_raw(self) -> HOT_KEY_MODIFIERS {
        HOT_KEY_MODIFIERS(self as u8 as u32)
    }
    const fn from_raw(raw: HOT_KEY_MODIFIERS) -> Option<Self> {
        match raw {
            MOD_SHIFT => Some(Self::Shift),
            MOD_CONTROL => Some(Self::Ctrl),
            MOD_ALT => Some(Self::Alt),
            MOD_WIN => Some(Self::Win),
            _ => None
        }
    }
    pub const fn name(self) -> &'static str {
        match self {
            Self::Shift => "Shift",
            Self::Ctrl => "Ctrl",
            Self::Alt => "Alt",
            Self::Win => "Win",
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ModifierSet(HOT_KEY_MODIFIERS);

impl ModifierSet {

    const fn as_raw(self) -> HOT_KEY_MODIFIERS { self.0 }

    pub const fn empty() -> Self { Self(HOT_KEY_MODIFIERS(0)) }

    pub const fn insert(&mut self, modifier: Modifier) {
        self.0.0 |= modifier.as_raw().0;
    }

    pub const fn remove(&mut self, modifier: Modifier) {
        self.0.0 &= !modifier.as_raw().0;
    }

    #[cfg(test)]
    pub const fn contains(&self, modifier: Modifier) -> bool {
        self.0.0 & modifier.as_raw().0 != 0
    }

}

impl Debug for ModifierSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl FromIterator<Modifier> for ModifierSet {
    fn from_iter<T: IntoIterator<Item=Modifier>>(iter: T) -> Self {
        let mut set = Self::empty();
        for modifier in iter {
            set.insert(modifier);
        }
        set
    }
}

impl IntoIterator for ModifierSet {
    type Item = Modifier;
    type IntoIter = ModifierSetIter;

    fn into_iter(self) -> Self::IntoIter {
        ModifierSetIter(self)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ModifierSetIter(ModifierSet);

impl Iterator for ModifierSetIter {
    type Item = Modifier;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Modifier::from_raw(HOT_KEY_MODIFIERS(1u32.unbounded_shl(self.0.0.0.trailing_zeros())))?;
        self.0.remove(next);
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyNameTextW, MapVirtualKeyW, MAPVK_VK_TO_VSC, MOD_NOREPEAT, VK_F1, VK_OEM_1, VK_SHIFT, VK_Y};
    use super::*;

    #[test]
    fn test_modifier_set() {
        let mut set = ModifierSet::empty();
        set.insert(Modifier::Shift);
        set.insert(Modifier::Ctrl);
        assert_eq!(set.contains(Modifier::Shift), true);
        assert_eq!(set.contains(Modifier::Ctrl), true);
        assert_eq!(set.contains(Modifier::Alt), false);
    }

    #[test]
    fn test_modifier_set_iter() {
        let mut iter = ModifierSetIter(ModifierSet(MOD_WIN | MOD_SHIFT | MOD_ALT | MOD_CONTROL | MOD_NOREPEAT));
        assert_eq!(iter.next(), Some(Modifier::Alt));
        assert_eq!(iter.next(), Some(Modifier::Ctrl));
        assert_eq!(iter.next(), Some(Modifier::Shift));
        assert_eq!(iter.next(), Some(Modifier::Win));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_vk_to_unicode() {
        let vk = VirtualKey::from(VK_Y);
        println!("Name: {}", vk.name());
    }
}