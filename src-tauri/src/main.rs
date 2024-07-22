// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn panic() -> String {
    panic!("This is a test for Sentry")
}

fn main() {
    dotenv::dotenv().ok();
    
    static SENTRY_DSN: &str = "SENTRY_DSN";

    println!("DSN: {}", SENTRY_DSN);

    let dsn = std::env::var(SENTRY_DSN).unwrap_or_else(|_| String::default());

    let client = tauri_plugin_sentry::sentry::init((
        dsn.clone(),
        tauri_plugin_sentry::sentry::ClientOptions {
            debug: true,
            release: tauri_plugin_sentry::sentry::release_name!(),
            ..Default::default()
        },
    ));
    let _guard = tauri_plugin_sentry::minidump::init(&client);

    tauri::Builder::default()
        .plugin(tauri_plugin_sentry::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![panic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
