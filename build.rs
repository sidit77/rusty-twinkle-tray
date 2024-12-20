use std::fs::{read, read_to_string};

use embedinator::{Icon, ResourceBuilder};

include!("assets/ids.rs");

const ICONS: &[(u16, &str)] = &[
    (APP_ICON, "assets/app.png"),
    (BRIGHTNESS_LIGHT_ICON, "assets/brightness_light.png"),
    (BRIGHTNESS_DARK_ICON, "assets/brightness_dark.png")
];

fn main() {
    let mut res = ResourceBuilder::from_env().add_manifest(read_to_string("assets/app.manifest").unwrap());

    println!("cargo:rerun-if-changed=assets/app.manifest");
    println!("cargo:rerun-if-changed=assets/ids.rs");

    for (id, name) in ICONS.iter().copied() {
        println!("cargo:rerun-if-changed={}", name);
        res = res.add_icon(id, Icon::from_png_bytes(read(name).unwrap()));
    }

    res.finish();

    println!("cargo:rerun-if-changed=build.rs");
}
