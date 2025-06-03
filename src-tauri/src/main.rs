mod libs;
mod tasks;
mod tauri_commands;

use crate::libs::utils::get_dir::get_default_download_dir;
use crate::tasks::handlers::app_settings_change_dir_task::app_settings_change_dir_task;
use crate::tasks::handlers::app_settings_get_task::app_settings_get_task;
use tasks::{
    handlers::app_settings_change_dir_task::AppSettings, processor::TaskProcessor, types::TaskType,
};
use tauri::{generate_context, Builder, Manager};
use tauri_plugin_store::StoreBuilder;

#[tokio::main]
async fn main() {
    println!("Started...");
    let default_settings = AppSettings {
        base_download_dir: get_default_download_dir(),
    };

    Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            // let window =
            //     tauri::WindowBuilder::new(app, "main", WindowUrl::App("index.html".into()))
            //         .title("Tauri App")
            //         .build()?;
            // window.open_devtools(); 
            // let window = app
            //     .get_webview_window("main")
            //     .expect("Window 'main' not found");
            // window.open_devtools();
            // println!("Window 'main' found, URL: {:?}", window.url());

            // --- StoreBuilder
            let store = StoreBuilder::new(app, "app_settings.json")
                .default(
                    "appSettings",
                    serde_json::to_value(&default_settings).unwrap(),
                )
                .build()
                .expect("failed to build store");

            // Register store in app state
            app.manage(store.clone());


            let app_handle = app.handle(); // for emit
            let mut processor = TaskProcessor::new(app_handle.clone());

            processor.register_handler(
                TaskType::TID_GET_MEDIAFILES_REQ,
                app_settings_get_task(store.clone()),
            );
            processor.register_handler(
                TaskType::TID_APP_SETTINGS_GET_REQ,
                app_settings_get_task(store.clone()),
            );
            processor.register_handler(
                TaskType::TID_APP_SETTINGS_CHANGE_DIR_REQ,
                app_settings_change_dir_task(store.clone()),
            );

            app.manage(processor);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_commands::cid_run_task,
            tauri_commands::cid_abort_task,
        ])
        .run(generate_context!())
        .expect("Error while running tauri application");
}
