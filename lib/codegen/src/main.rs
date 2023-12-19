mod attributes;
mod utils;
mod whitelist;
mod exporter;
mod filter;
mod constructors;

use std::collections::{HashMap, HashSet};
use std::ops::Sub;
use std::path::{Path, PathBuf};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use windows_bindgen::bindgen;
use crate::attributes::strip_attributes;
use crate::constructors::generate_constructors;
use crate::exporter::generate_export;
use crate::filter::should_process_file;
use crate::whitelist::apply_whitelist;

#[derive(Default, Serialize, Deserialize)]
struct Cache {
    classes: HashSet<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WhiteList {
    Subset(HashSet<String>),
    All(bool)
}

impl Default for WhiteList {
    fn default() -> Self {
        Self::All(false)
    }
}

impl WhiteList {
    pub fn is_enabled(&self, name: &str) -> bool {
        match self {
            WhiteList::Subset(enabled) => enabled.contains(name),
            WhiteList::All(r) => *r
        }
    }

    pub fn all_enabled(&self) -> bool {
        matches!(self, WhiteList::All(true))
    }

    pub fn add(&mut self, items: HashSet<String>) {
        match self {
            WhiteList::Subset(set) => set.extend(items.into_iter()),
            WhiteList::All(false) => *self = WhiteList::Subset(items),
            WhiteList::All(true) => {}
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
struct Config {
    temp_dir: PathBuf,
    classes: HashSet<String>,
    features: HashSet<String>,
    constructors: HashSet<String>,
    white_list: HashMap<String, WhiteList>,
    reexports: HashSet<String>
}

fn main() {

    let config = {
        let path = std::env::args().skip(1).next().expect("Missing path to target dir");
        std::env::set_current_dir(path).expect("Failed to go to target dir");
        let content = std::fs::read_to_string("Codegen.toml").expect("Failed to read config file");

        toml::from_str::<Config>(&content)
            .expect("Failed to parse config file")
            .with_expanded_features()
    };

    {
        let mut cached: Cache = std::fs::read_to_string(config.temp_dir.join("cache.toml"))
            .map_err(|err| println!("Failed to read cache file: {err}"))
            .ok()
            .and_then(|f| toml::from_str(&f)
                .map_err(|err| println!("Failed to parse cache file: {err}"))
                .ok())
            .unwrap_or_default();

        if cached.classes != config.classes {
            println!("Regenerating source files...");
            if config.temp_dir.exists() {
                std::fs::remove_dir_all(&config.temp_dir).unwrap();
            }
            std::fs::create_dir_all(config.temp_dir.join("src")).unwrap();
            std::fs::write(config.temp_dir.join("Cargo.toml"), "").unwrap();

            let mut args = vec![
                String::from("--out"),
                config.temp_dir.join("src").join("lib.rs").to_str().unwrap().to_string(),
                String::from("--config"),
                String::from("package"),
            ];
            for class in &config.classes {
                args.push(String::from("--filter"));
                args.push(class.clone());
            }
            bindgen(args).unwrap();
            cached.classes = config.classes.clone();
            std::fs::write(config.temp_dir.join("cache.toml"), toml::to_string(&cached).unwrap()).unwrap();
        }

    }

    let src_dir = config.temp_dir.join("src");
    let target_dir = PathBuf::from("src");

    if target_dir.join("Windows").exists() {
        std::fs::remove_dir_all(target_dir.join("Windows")).unwrap();
    }
    let encountered = WalkDir::new(&src_dir)
        .into_iter()
        .map(Result::unwrap)
        .par_bridge()
        .filter(|e| e.file_type().is_file())
        .map(|e| e
            .path()
            .strip_prefix(&src_dir)
            .unwrap()
            .to_path_buf())
        .flat_map(|e| transform(
            src_dir.join(&e),
            target_dir.join(&e),
            &config
        ))
        .collect::<HashSet<String>>();

    let disabled_features = {
        let mut f = Vec::from_iter(encountered.sub(&config.features).into_iter());
        f.sort();
        f
    };
    println!("disabled features: {:#?}", disabled_features);

    config
        .reexports
        .iter()
        .for_each(|m| generate_export(m, &target_dir));

    //transform(
    //    PathBuf::from(base_path).join("Windows\\UI\\Xaml\\mod.rs"),
    //    PathBuf::from(target_path).join("Windows\\UI\\Xaml\\mod.rs"),
    //    &config,
    //    &mut encountered
    //);



}

fn transform<I: AsRef<Path>, O: AsRef<Path>>(in_file: I, out_file: O, config: &Config) -> HashSet<String> {
    if !should_process_file(in_file.as_ref(), &config.features) {
        println!("Skipped: {:?}", in_file.as_ref());
        return HashSet::new();
    }

    let src = std::fs::read_to_string(in_file.as_ref()).unwrap();
    let mut file = syn::parse_file(&src).unwrap();

    apply_whitelist(&mut file.items, &config.white_list);

    let mut encountered = HashSet::new();
    strip_attributes(&mut file.items, &config.features, &mut encountered);

    generate_constructors(&mut file.items, &config.constructors);
    //print_items(file.items);
    //delete_impl(&mut file.items);
    std::fs::create_dir_all(out_file.as_ref().parent().unwrap()).unwrap();
    std::fs::write(out_file, prettyplease::unparse(&file)).unwrap();

    println!("Processed: {:?}", in_file.as_ref());

    encountered
}

impl Config {
    fn with_expanded_features(mut self) -> Self {
        let mut expanded = HashSet::new();
        for feature in &self.features {
            let mut current = String::new();
            for comp in feature.split("_") {
                if !current.is_empty() {
                    current.push('_');
                }
                current.push_str(comp);
                expanded.insert(current.clone());
            }
        }
        self.features = expanded;
        self
    }
}