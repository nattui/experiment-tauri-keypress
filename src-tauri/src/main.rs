// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use tauri::AppHandle;
use tauri::Manager;

#[tauri::command]
fn play_sound(app: AppHandle) {
    // Get the resource path
    let resource_path = app
        .path()
        .resource_dir()
        .unwrap()
        .join("resources")
        .join("sound.wav");

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load and decode the sound file
    let file = File::open(resource_path).unwrap();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).unwrap();

    // Play the sound
    sink.append(source);
    sink.sleep_until_end();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
