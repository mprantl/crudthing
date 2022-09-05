#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//use crate::crud::*;
use std::path::PathBuf;
use tauri::Manager;

pub mod crud;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct DB_CONNECTION(PathBuf);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app
                .path_resolver()
                .resolve_resource("assets/")
                .expect("Something went wrong while setting up #1")
                .join("LogBook.db");
            app.manage(DB_CONNECTION(path));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crud::create::create,
            crud::delete::delete_worker,
            crud::read::check_worker_id,
            crud::read::fetch_table,
            crud::update::update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
