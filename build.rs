use std::path::{Path, PathBuf};
include!("assets/ids.rs");

const ICONS: &[(u16, &str)] = &[
    (APP_ICON, "app.png"),
    (BRIGHTNESS_LIGHT_ICON, "brightness_light.png"),
    (BRIGHTNESS_DARK_ICON, "brightness_dark.png")
];

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let assets_path = Path::new("assets");
        let icon_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

        let mut res = tauri_winres::WindowsResource::new();
        res.set_manifest(include_str!("assets/app.manifest"));
        println!("cargo:rerun-if-changed=assets/app.manifest");
        println!("cargo:rerun-if-changed=assets/ids.rs");

        for (id, name) in ICONS.iter().copied() {
            let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
            let file = std::fs::File::open(assets_path.join(name)).unwrap();
            let image = ico::IconImage::read_png(file).unwrap();
            let icon_path = icon_path.join(name).with_extension("ico");
            icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
            icon_dir
                .write(std::fs::File::create(&icon_path).unwrap())
                .unwrap();
            res.set_icon_with_id(icon_path.to_str().unwrap(), &format!("{}", id));
            println!("cargo:rerun-if-changed=assets/{}", name);
        }

        res.compile().unwrap();
    }
    println!("cargo:rerun-if-changed=build.rs");

}
