use include_dir::{include_dir, Dir};
use serde_json::Value;
use std::collections::HashMap;

const ASSETS_DIR: Dir = include_dir!("assets");

fn get_all_files() -> HashMap<String, String> {
    let mut file_contents: HashMap<String, String> = HashMap::new();
    for file in ASSETS_DIR.files().filter(|f| f.path().extension().unwrap_or_default().to_str().unwrap() == "json") {
        let friendly_name: String = file.path().file_stem().unwrap().to_str().unwrap().into();
        file_contents.insert(friendly_name, String::from(file.contents_utf8().unwrap()));
    }
    file_contents
}

pub fn get_all_data() -> HashMap<String, Value> {
    get_all_files()
        .iter()
        .map(|(key, contents)| {
            let parsed: Value = serde_json::from_str(contents).unwrap();
            (key.to_owned(), parsed)
        })
        .collect()
}
