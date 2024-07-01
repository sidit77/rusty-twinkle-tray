use std::collections::HashSet;
use std::path::Path;

pub fn should_process_file(file: &Path, enabled: &HashSet<String>) -> bool {
    match file.file_name().unwrap() {
        name if name == "mod.rs" => {
            let package = generate_package_name(file);
            package.is_empty() || enabled.contains(&package)
        }
        name if name == "impl.rs" => enabled.contains("implement"),
        _ => unreachable!()
    }
}

fn generate_package_name(file: &Path) -> String {
    let mut comps: Vec<String> = file
        .iter()
        .rev()
        .skip(1)
        .map(|s| s.to_str().unwrap())
        .take_while(|c| *c != "Windows")
        .map(String::from)
        .collect();
    comps.reverse();
    comps.join("_")
}
