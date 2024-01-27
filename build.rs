
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let icon_path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("icon.ico");
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
        let file = std::fs::File::open("assets/icon.png").unwrap();
        let image = ico::IconImage::read_png(file).unwrap();
        icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
        icon_dir
            .write(std::fs::File::create(&icon_path).unwrap())
            .unwrap();

        let mut res = tauri_winres::WindowsResource::new();
        res.set_manifest(include_str!("assets/app.manifest"));
        res.set_icon(icon_path.to_str().unwrap());
        res.compile().unwrap();
    }
    println!("cargo:rerun-if-changed=assets/app.manifest");
    println!("cargo:rerun-if-changed=assets/icon.png");
    println!("cargo:rerun-if-changed=build.rs");

}
