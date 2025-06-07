use std::{sync::{Arc, Mutex}, fs::{read_dir, File}, path::{PathBuf, Path}, collections::HashMap, io::Read};

use serde_json::{self, Value};
use serde::Serialize;
use tauri::Manager;
use zip::ZipArchive;

use crate::LoadingEvent;

#[derive(Serialize)]
#[derive(Clone)]
pub struct ModFile {
    name: String,
    is_jar: bool,
    loader: Option<String>,
    mc_version: Option<String>,
}

#[derive(Default)]
pub struct ModFileCacheRef(pub Arc<Mutex<ModFileCache>>);

#[derive(Default)]
pub struct ModFileCache(pub HashMap<String, ModFile>);

pub fn fill_vector_from_dir(
    dir_path: &Path,
    vec: &mut Vec<ModFile>,
    filename_filter: &str,
    cache: &mut ModFileCache,
    app_handle: &tauri::AppHandle) {

    for entry in read_dir(dir_path).expect("Coulding read dir") {
        let dir_entry = entry.expect("Couldnt read entry");
        let file_name_os = dir_entry.file_name();
        let file_name = file_name_os.into_string().expect("Couldnt convert");
        
        if file_name.to_lowercase().contains(filename_filter) {
            let _ = app_handle.emit_all(
                "loading",
                LoadingEvent{ text: Some(file_name.clone()), is_loading: true}
            );

            let file_to_send = prepare_modfile(file_name, dir_entry.path(), cache);    
            vec.push(file_to_send);
        }
    }
}

fn prepare_modfile(file_name: String, path: PathBuf, cache: &mut ModFileCache) -> ModFile {
    let is_jar = Path::new(&file_name).extension().unwrap().to_str().unwrap() == "jar";

    let mut file_to_send = ModFile { name: file_name.clone(), is_jar: is_jar, loader: None, mc_version: None };

    if is_jar {
        if cache.0.contains_key(&file_name) {
            let cached_mod_file = cache.0.get(&file_name).unwrap();
            file_to_send.loader = cached_mod_file.loader.clone();
            file_to_send.mc_version = cached_mod_file.mc_version.clone();
        } else {
            fill_mcmod_details(path, &mut file_to_send);

            cache.0.insert(file_name.clone(), file_to_send.clone());
        }
    }

    return file_to_send;
}

fn fill_mcmod_details(path: PathBuf, file_to_send: &mut ModFile) {
    let mut zip_file = ZipArchive::new(File::open(path.clone()).unwrap()).unwrap();

    // fabric scope
    {
        let fabricmodjson = zip_file.by_name("fabric.mod.json");

        if fabricmodjson.is_ok() {
            // Fabric mod
            file_to_send.loader = Some(String::from("Fabric"));

            println!("Fabric {}", file_to_send.name);
            
            let mut buf = String::new();
            fabricmodjson.unwrap().read_to_string(&mut buf).unwrap();
            let fabricmod_json: Value = serde_json::from_str(&buf).unwrap();
            
            if fabricmod_json.get("schemaVersion").unwrap().as_i64().unwrap() == 1 {
                let depends_json: &Value = fabricmod_json.get("depends").unwrap();
                let minecraft_version = depends_json.get("minecraft");
                if minecraft_version.is_none() {
                    return;
                }
                file_to_send.mc_version = Some(String::from(
                    minecraft_version.unwrap().as_str().unwrap_or("???")
                ));
            }
        }
    }
    // forge scope
    {
        let mcmodinfo = zip_file.by_name("mcmod.info");

        if mcmodinfo.is_ok() {
            // Forge mod
            file_to_send.loader = Some(String::from("Forge"));

            println!("Forge {}", file_to_send.name);

            let mcmod_info: Value = serde_json::from_reader(mcmodinfo.unwrap()).expect("Couldnt parse json");
            
            let first_elem = mcmod_info.get(0);

            if first_elem.is_none() {
                return
            }
            
            let mcversion_elem = first_elem.unwrap().get("mcversion");

            if mcversion_elem.is_none() {
                return
            }

            file_to_send.mc_version = Some(
                String::from(mcversion_elem.unwrap().as_str().unwrap())
            );
        }
    }
}