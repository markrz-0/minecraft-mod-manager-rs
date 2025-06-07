// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mod_info;
mod mc_forge;
mod mc_fabric;
mod common;


use std::{sync::{Arc, Mutex}, fs::{create_dir_all, rename, remove_file, copy}};
use std::path::Path;
use serde_json;

use tauri::{State, Manager, api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind}};

use mod_info::{ModFile, ModFileCache, ModFileCacheRef};

use crate::common::is_java_installed;

#[derive(Default)]
struct Filter(Arc<Mutex<String>>);

#[derive(Clone, serde::Serialize)]
pub struct LoadingEvent {
    text: Option<String>,
    is_loading: bool
}

#[derive(Clone, serde::Serialize)]
pub struct ToastEvent {
    text: String
}

fn main() {

    //client::start_broadcast_listener_thread();

    tauri::Builder::default()
        .manage(Filter(Default::default()))
        .manage(ModFileCacheRef(Default::default()))
        .invoke_handler(tauri::generate_handler![
            update_search_filter,
            move_file,
            del_file,
            import_file,
            refresh_content,
            install_forge,
            get_mc_versions,
            get_forge_versions,
            install_fabric])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
async fn install_forge(app_handle: tauri::AppHandle, mc_version: String, forge_version: String) -> Result<(), ()> {
    if !is_java_installed() {
        let _  = app_handle.emit_all("toast", ToastEvent{ text: String::from("Java not installed (JAVA_HOME not found)")});
        return Ok(())
    }
    
    let _ = app_handle.emit_all(
        "loading",
        LoadingEvent{ text: None, is_loading: true }
    );
    println!("INSTALL FORGE CMD");
    mc_forge::download_and_run_forge_binary(mc_version.as_str(), forge_version.as_str()).await.unwrap();
    let _ = app_handle.emit_all(
        "loading",
        LoadingEvent{ text: None, is_loading: false }
    );
    Ok(())
}

#[tauri::command]
async fn install_fabric(app_handle: tauri::AppHandle) -> Result<(), ()> {
    if !is_java_installed() {
        let _ = app_handle.emit_all("toast", ToastEvent{ text: String::from("Java not installed (JAVA_HOME not found)")});
        return Ok(())
    }

    let _ = app_handle.emit_all(
        "loading",
        LoadingEvent{ text: None, is_loading: true }
    );
    println!("INSTALL FABRIC CMD");
    mc_fabric::download_and_run_fabric_binary().await.unwrap();
    let _ = app_handle.emit_all(
        "loading",
        LoadingEvent{ text: None, is_loading: false }
    );
    Ok(())
}

#[tauri::command]
async fn get_mc_versions() -> Result<Vec<String>, ()> {
    println!("MC VERSIONS");
    let out = mc_forge::fetch_mc_versions().await.unwrap();
    Ok(out)
}

#[tauri::command]
async fn get_forge_versions(mc_version: String) -> Result<Vec<String>, ()> {
    println!("FORGE VERSIONS");
    let out = mc_forge::fetch_forge_versions(mc_version).await.unwrap();
    Ok(out)
}

#[tauri::command]
fn update_search_filter(val: String, filter: State<'_, Filter>) {
    let filter_lock_result = filter.0.lock();
    if filter_lock_result.is_err() {
        return
    }
    let mut filter_val = filter_lock_result.unwrap();
    *filter_val = val;
}

#[tauri::command]
fn move_file(file: String, to: String) {
    let file_path_obj = Path::new(file.as_str());
    // maybe remove those unwraps?? idk
    let file_stem = file_path_obj.file_stem().unwrap().to_str().unwrap();
    let file_ext = file_path_obj.extension().unwrap().to_str().unwrap();
    
    let installed_path_str = get_installed_path_str();
    let available_path_str = get_available_path_str();

    let from_path_prefix = if to == "available" { installed_path_str.clone() }  else {  available_path_str.clone() };
    let to_path_prefix = if to == "available" { available_path_str.clone() }  else {  installed_path_str.clone() };

    let from_path_str = from_path_prefix + "\\" + &file;
    let mut to_path_str = to_path_prefix.clone() + "\\" + &file;

    let from_path = Path::new(from_path_str.as_str());
    let mut to_path = Path::new(to_path_str.as_str());

    let mut idx = 1;
    while to_path.exists() {
        
        to_path_str = to_path_prefix.clone() + "\\" + &file_stem+ " (" + &idx.to_string() + ")" + &file_ext;

        to_path = Path::new(to_path_str.as_str());
        idx += 1;
    }

    let rename_result = rename(from_path, to_path);
    if rename_result.is_err() {
        MessageDialogBuilder::new("Błąd!", "Nie można przenieść pliku. Zamknij Minecraft.")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .show(|_f|());
    }
}

#[tauri::command]
fn del_file(file: String, from: String) {
    let installed_path_str = get_installed_path_str();
    let available_path_str = get_available_path_str();

    let from_path_prefix = if from == "installed" { installed_path_str }  else {  available_path_str };
    let from_path_str = from_path_prefix + "\\" + &file;
    let from_path = Path::new(from_path_str.as_str());

    let remove_file_result = remove_file(from_path);
    if remove_file_result.is_err() {
        MessageDialogBuilder::new("Błąd!", "Nie można usunąć pliku. Zamknij Minecraft")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .show(|_f|()); // the thing inside is rust way of saying (_f: boolean) => {};
    }
}

#[tauri::command]
fn import_file(file_path: String) {
    let from_path = Path::new(file_path.as_str());
    // those unwraps...
    let file_stem = from_path.file_stem().unwrap().to_str().unwrap();
    let file_ext = from_path.extension().unwrap().to_str().unwrap();
    let file = String::from(file_stem) + "." + &file_ext;

    let available_path_str = get_available_path_str();

    let mut to_path_str = available_path_str.clone() + "\\" + &file;

    let mut to_path = Path::new(to_path_str.as_str());

    let mut idx = 1;
    while to_path.exists() {
        
        to_path_str = available_path_str.clone() + "\\" + &file_stem + " (" + &idx.to_string() + ")" + &file_ext;

        to_path = Path::new(to_path_str.as_str());
        idx += 1;
    }

    let from_drive = from_path.to_str().unwrap().as_bytes()[0] as char;
    let to_drive = to_path.to_str().unwrap().as_bytes()[0] as char;

    let is_move_successful;
    
    if from_drive == to_drive {
        let rename_result = rename(from_path, to_path);
        is_move_successful = rename_result.is_ok();
    } else {
        let copy_result = copy(from_path, to_path);
        is_move_successful = copy_result.is_ok();
    }

    if !is_move_successful {
        MessageDialogBuilder::new("Błąd!", "Nie można zaimportować pliku")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .show(|_f|()); // the thing inside is rust way of saying (_f: boolean) => {};
    }
}

#[tauri::command]
async fn refresh_content(filter: State<'_, Filter>, cache: State<'_, ModFileCacheRef>, app_handle: tauri::AppHandle) -> Result<(), ()> {
    let filter_val = filter.0.lock().unwrap();
    let mut cache_val = cache.0.lock().unwrap();
    send_folder_contents(filter_val.clone(), &mut cache_val, app_handle);
    Ok(())
}

fn get_installed_path_str() -> String {
    #[cfg(unix)]
    let app_data = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");

    let installed_path_string = app_data + "\\.minecraft\\mods";

    return installed_path_string;
}

fn get_available_path_str() -> String {
    #[cfg(unix)]
    let app_data = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");

    let available_path_string = app_data + "\\modmanager2.0\\available";

    return available_path_string;
}

fn send_folder_contents(filter: String, cache: &mut ModFileCache, app_handle: tauri::AppHandle) {
    let mut installed_mods_list: Vec<ModFile> = vec![];
    let installed_path_str = get_installed_path_str();
    let installed_path = Path::new(installed_path_str.as_str());

    create_dir_all(installed_path).expect("Couldnt create dirs");
    mod_info::fill_vector_from_dir(installed_path, &mut installed_mods_list, filter.as_str(), cache, &app_handle);

    let mut available_mods_list: Vec<ModFile> = vec![];
    let available_path_str = get_available_path_str();
    let available_path = Path::new(available_path_str.as_str());

    create_dir_all(available_path).expect("Couldnt create dirs");
    mod_info::fill_vector_from_dir(available_path, &mut available_mods_list, filter.as_str(), cache, &app_handle);

    let payload = serde_json::json!({ "installed": installed_mods_list, "available": available_mods_list });
    app_handle.emit_all("folder_contents", payload ).expect("Couldnt emit");
    let _ = app_handle.emit_all(
        "loading",
        LoadingEvent{ text: None, is_loading: false }
    );
}