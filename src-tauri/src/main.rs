mod libs;
mod tasks;

use crate::libs::utils::get_dir::get_default_download_dir;
use crate::tasks::handlers::app_settings_change_dir_task::app_settings_change_dir_task;
use crate::tasks::handlers::app_settings_get_task::app_settings_get_task;
use tasks::{
    handlers::app_settings_change_dir_task::AppSettings, processor::TaskProcessor, types::TaskType, tauri_commands,
};
use tauri::{generate_context, Builder, Manager};
use tauri_plugin_store::StoreBuilder;

fn main() {
    println!("Started...");
    let default_settings = AppSettings {
        base_download_dir: get_default_download_dir(),
    };

    Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            // Создаем store с дефолтными настройками
            let store = StoreBuilder::new(app, "app_settings.json")
                .default(
                    "appSettings",
                    serde_json::to_value(&default_settings).unwrap(),
                )
                .build()
                .expect("failed to build store");

            // Регистрируем store как глобальное состояние
            app.manage(store.clone());

            let app_handle = app.handle();
            let mut processor = TaskProcessor::new(app_handle.clone());

            // Регистрируем обработчики задач, передавая store
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

            // Регистрируем процессор задач
            app.manage(processor);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_commands::cid_run_task,
            tauri_commands::cid_abort_task,
            tauri_commands::cid_app_settings_get,
        ])
        .run(generate_context!())
        .expect("Error while running tauri application");
}
