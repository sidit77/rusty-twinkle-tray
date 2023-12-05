use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn generate_export<P: AsRef<Path>>(module: &str, base: P) {
    let mut export = String::from("pub use windows::");
    let mut path = base.as_ref().join("Windows");
    for comp in module.split("_") {
        export.push_str(comp);
        export.push_str("::");

        path.push(comp);
        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
            std::fs::write(path.join("mod.rs"), "").unwrap();
            append(path.parent().unwrap().join("mod.rs"), format!("pub mod {comp};")).unwrap();
        }
    }
    export.push_str("*;\n");
    path.push("mod.rs");


    append(path, export).unwrap();
}

fn append<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> std::io::Result<()> {
    OpenOptions::new()
        .append(true)
        .open(path)?
        .write_all(content.as_ref())
}