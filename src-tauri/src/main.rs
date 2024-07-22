// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let dsn = std::env::var("https://c71cc8cacbe75cb8da3839e6ef10df70@o4507644748759040.ingest.de.sentry.io/4507644801712208").unwrap_or_else(|_| String::new());

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
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
