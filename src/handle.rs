use std::collections::BTreeMap;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs};

use serde_json;

const DOT_DIR: &str = ".tart";

pub type Message = String;

pub fn init(dir: Option<&PathBuf>) -> Message {
    let mut actual_path = env::current_dir().unwrap();
    if let Some(given_dir) = dir {
        actual_path = given_dir.to_path_buf();
    }
    actual_path.push(DOT_DIR);

    if actual_path.exists() {
        return format!(
            "tart repository already exists at `{}`",
            &actual_path.to_str().unwrap()
        );
    }

    match fs::create_dir(&actual_path) {
        // attempts to init dir
        Ok(()) => {
            // init `.tart` contents
            let mut file = fs::File::create(&actual_path.join("project.json")).unwrap();

            let mut map = BTreeMap::new();
            map.insert("name".to_string(), 1.0);
            map.insert("y".to_string(), 2.0);

            let json_text = serde_json::to_string(&map).unwrap();
            file.write_all(json_text.as_bytes()).unwrap();
            // ======================================

            format!(
                "initialized tart at `{}`",
                get_absolute_path(&actual_path).unwrap()
            ) // path should exist after create_dir
        }
        Err(_) => format!(
            "failed to initialize tart at `{}`",
            &actual_path.to_str().unwrap()
        ), // path does not exist
    }
}

pub fn destroy(dir: Option<&PathBuf>) -> Message {
    let mut actual_path = env::current_dir().unwrap();
    if let Some(given_dir) = dir {
        actual_path = given_dir.to_path_buf();
    }
    actual_path.push(DOT_DIR);

    match get_absolute_path(&actual_path) {
        Ok(path) => {
            // path exists
            match fs::remove_dir_all(&path) {
                // attempts to remove dir
                Ok(()) => format!("destroyed tart at `{}`", &path),
                Err(_) => format!("failed to destroy tart at `{}`", &path),
            }
        }
        Err(err_message) => err_message, // path does not exist
    }
}

pub fn board(name: Option<&String>) -> Message {
    match name {
        Some(val) => {
            format!("some {}", val)
        }
        None => {
            format!("none")
        }
    }
}

fn get_absolute_path(path: &PathBuf) -> Result<Message, Message> {
    match fs::canonicalize(&path) {
        Ok(pathbuf) => {
            let mut path_str = pathbuf.to_str().unwrap();

            if cfg!(windows) {
                path_str = path_str.trim_start_matches(r"\\?\");
            }

            Ok(path_str.to_string())
        }
        Err(_) => Err(format!("path `{}` not found", &path.to_str().unwrap())),
    }
}
