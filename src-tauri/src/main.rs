// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use doc_anal::decoder::WordDocument;
use json::{object, JsonValue};
use std::{fs::File, path::PathBuf, sync::Mutex};
use tauri::{Manager, State};
use util::save_recent_open;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            read_file,
            read_comparison_file,
            get_logical_view,
            get_physical_view,
            get_compare_physical_view,
            get_compare_logical_view,
            get_recent_open,
            get_recent_compare,
            get_file_name,
            get_comparison_file_name,
            get_compare_physical_view,
            get_utf8_interpreted_bytes,
        ])
        // open the devtools
        .setup(|app| {
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn read_file(file_path: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let file = File::open(&file_path).map_err(|err| err.to_string())?;
    save_recent_open(&file_path);
    let word_doc = WordDocument::read_file(file).map_err(|err| err.to_string())?;

    let mut state = state.lock().unwrap();
    state.word_doc = Some(word_doc);
    // state.file_name = Some(
    //     Path::new(&file_path)
    //         .file_name()
    //         .unwrap()
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    // );
    state.ref_file_path = Some(PathBuf::from(file_path));

    Ok(())
}

#[tauri::command]
fn read_comparison_file(
    file_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let file = File::open(&file_path).map_err(|err| err.to_string())?;
    save_recent_open(&file_path);
    // if there is a ref file, save compare
    match state.lock().unwrap().ref_file_path.as_ref() {
        Some(ref_path) => {
            util::save_recent_compare(&ref_path.to_str().unwrap(), &file_path);
        }
        None => {}
    }
    let word_doc = WordDocument::read_file(file).map_err(|err| err.to_string())?;

    let mut state = state.lock().unwrap();
    state.compare_word_doc = Some(word_doc);
    // state.compare_file_name = Some(
    //     Path::new(&file_path)
    //         .file_name()
    //         .unwrap()
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    // );
    state.comp_file_path = Some(PathBuf::from(file_path));

    Ok(())
}

#[tauri::command]
fn get_recent_open() -> Vec<String> {
    let paths = util::get_recent_open();

    // create {name: string, path: string} objects
    paths
        .iter()
        .map(|path| {
            let name = path.file_name().unwrap().to_str().unwrap();
            let path = path.to_str().unwrap();
            object! {
                name: name,
                path: path
            }
            .to_string()
        })
        .collect()
}

#[tauri::command]
fn get_recent_compare() -> Vec<String> {
    let paths = util::get_recent_compare();

    // create {name: string, path: string} objects
    paths
        .iter()
        .map(|(ref_path, comp_path)| {
            let ref_name = ref_path.file_name().unwrap().to_str().unwrap();
            let ref_path = ref_path.to_str().unwrap();
            let comp_name = comp_path.file_name().unwrap().to_str().unwrap();
            let comp_path = comp_path.to_str().unwrap();

            let ref_object = object! {
                name: ref_name,
                path: ref_path
            };
            let comp_object = object! {
                name: comp_name,
                path: comp_path
            };

            let object: JsonValue = vec![ref_object, comp_object].into();
            object.to_string()
        })
        .collect()
}

#[tauri::command]
fn get_file_name(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().unwrap();
    Ok(state.file_name().clone().unwrap_or_default())
}

#[tauri::command]
fn get_comparison_file_name(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().unwrap();
    Ok(state.compare_file_name().clone().unwrap_or_default())
}

#[tauri::command]
fn get_logical_view(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().unwrap();
    let wor_doc = state.word_doc.as_ref().unwrap();
    let return_thing = wor_doc.to_json_logical();
    Ok(return_thing.to_string())
}

#[tauri::command]
fn get_physical_view(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().unwrap();
    let wor_doc = state.word_doc.as_ref().unwrap();
    let return_thing = wor_doc.to_json_physical();
    Ok(return_thing.to_string())
}

#[tauri::command]
fn get_compare_physical_view(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().unwrap();
    let wor_doc = state.compare_word_doc.as_ref().unwrap();
    let comparison_word_doc = state.word_doc.as_ref().unwrap();

    let return_thing = wor_doc.compare_to_physical(comparison_word_doc);
    Ok(return_thing.to_string())
}

#[tauri::command]
fn get_compare_logical_view(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().unwrap();
    let wor_doc = state.compare_word_doc.as_ref().unwrap();
    let comparison_word_doc = state.word_doc.as_ref().unwrap();

    let return_thing = wor_doc.compare_to_logical(comparison_word_doc);
    Ok(return_thing.to_string())
}

// only makes sense for use with physical
#[tauri::command]
fn get_utf8_interpreted_bytes(
    word_doc: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let state = state.lock().unwrap();
    let word_doc = if word_doc == "analysis" {
        state.word_doc.as_ref().unwrap()
    } else if word_doc == "comparison" {
        state.compare_word_doc.as_ref().unwrap()
    } else {
        return Err("Invalid argument".to_string());
    };

    let output_vec = word_doc
        .get_physical_sructures()
        .iter()
        .map(|structure| util::bytes_to_utf8_or_bullet(&structure.bytes))
        .collect();

    Ok(output_vec)
}

struct AppState {
    // file_name: Option<String>,
    ref_file_path: Option<PathBuf>,
    word_doc: Option<WordDocument>,
    // compare_file_name: Option<String>,
    comp_file_path: Option<PathBuf>,
    compare_word_doc: Option<WordDocument>,
}

impl AppState {
    const fn new() -> Self {
        AppState {
            // compare_file_name: None,
            // file_name: None,
            ref_file_path: None,
            comp_file_path: None,
            word_doc: None,
            compare_word_doc: None,
        }
    }

    fn file_name(&self) -> Option<String> {
        self.ref_file_path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }

    fn compare_file_name(&self) -> Option<String> {
        self.comp_file_path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }
}

mod util {
    use std::{io::Read, path::PathBuf};

    pub fn save_recent_open(new_path: &str) {
        let path = create_or_get_app_data();
        let file_path = path.join("recent_opened.txt");

        // Open the file for reading and writing, create it if it doesn't exist
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)
            .unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Check if the new path is already in the contents
        if !contents.lines().any(|line| line == new_path) {
            // If not, append the new path to the contents
            contents.push_str(new_path);
            contents.push('\n');

            // Write the updated contents back to the file
            std::fs::write(&file_path, contents).unwrap();
        }
    }

    // save 2 paths that are being compared
    pub fn save_recent_compare(ref_path: &str, comp_path: &str) {
        let path = create_or_get_app_data();
        let file_path = path.join("recent_compare.txt");

        // Open the file for reading and writing, create it if it doesn't exist
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)
            .unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Check if the new path is already in the contents
        if !contents
            .lines()
            .any(|line| line == format!("{}|{}", ref_path, comp_path))
        {
            // If not, append the new path to the contents
            contents.push_str(&format!("{}|{}\n", ref_path, comp_path));

            // Write the updated contents back to the file
            std::fs::write(&file_path, contents).unwrap();
        }
    }

    pub fn get_recent_open() -> Vec<PathBuf> {
        let path = create_or_get_app_data();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("recent_opened.txt"))
            .unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut recent_opened = contents.lines().collect::<Vec<_>>();
        recent_opened.reverse();
        recent_opened.iter().map(|s| PathBuf::from(s)).collect()
    }

    pub fn get_recent_compare() -> Vec<(PathBuf, PathBuf)> {
        let path = create_or_get_app_data();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("recent_compare.txt"))
            .unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut recent_compare = contents.lines().collect::<Vec<_>>();
        recent_compare.reverse();
        recent_compare
            .iter()
            .map(|s| {
                let mut split = s.split('|');
                (
                    PathBuf::from(split.next().unwrap()),
                    PathBuf::from(split.next().unwrap()),
                )
            })
            .collect()
    }

    fn create_or_get_app_data() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap();
        path.pop();
        path.push("app_data");
        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
        }
        path
    }

    pub fn bytes_to_utf8_or_bullet(bytes: &Vec<u8>) -> String {
        bytes
            .iter()
            .map(|b| match std::str::from_utf8(&[*b]) {
                Ok(s) => s.to_string(),
                Err(_) => "•".to_string(),
            })
            .collect::<String>()
    }

    pub fn _bytes_to_ascii_or_bullet(bytes: &Vec<u8>) -> String {
        bytes
            .iter()
            .map(|&b| match std::str::from_utf8(&[b]) {
                Ok(s) => {
                    let c = s.chars().next().unwrap();
                    if c.is_alphanumeric() || c.is_ascii_punctuation() {
                        s.to_string()
                    } else {
                        "•".to_string()
                    }
                }
                Err(_) => "•".to_string(),
            })
            .collect::<String>()
    }
}
