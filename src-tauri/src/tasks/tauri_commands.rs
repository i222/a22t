use crate::tasks::processor::TaskProcessor;
use crate::tasks::types::TaskInput;

#[tauri::command]
pub async fn cid_run_task(
    task: TaskInput,
    processor: State<'_, TaskProcessor>,
) -> Result<String, String> {
    println!("[cid_run_task] received task: {:?}", task);
    let res = processor.run_task(task).await;
    // println!("[cid_run_task] result: {:?}", res);
    res
}

#[tauri::command]
pub async fn cid_abort_task(
    taskId: String,
    processor: State<'_, TaskProcessor>,
) -> Result<(), String> {
    println!("[cid_abort_task] aborting task with id: {}", taskId);
    processor.abort_task(&taskId).await;
    // println!("[cid_abort_task] abort requested for task id: {}", taskId);
    Ok(())
}

use serde::{Deserialize, Serialize};
use tauri::{State, Wry};
use tauri_plugin_store::Store;
// use tauri::EventLoopMessage;
use std::sync::Arc;
use crate::libs::utils::get_dir::get_default_download_dir;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub base_download_dir: String,
}

#[tauri::command]
pub async fn cid_app_settings_get(
    store: State<'_, Arc<Store<Wry>>>,
) -> Result<serde_json::Value, String> {
    println!("[cid_app_settings_get] income");

    let settings: Option<AppSettings> = store.get("appSettings").and_then(|val| {
        if val.is_null() {
            None
        } else {
            serde_json::from_value(val).ok()
        }
    });

    println!("[cid_app_settings_get] settings {:?}", settings);

    let result = settings.unwrap_or_else(|| AppSettings {
        base_download_dir: get_default_download_dir(),
    });
    println!("[cid_app_settings_get] result {:?}", result);
    serde_json::to_value(result).map_err(|e| format!("Serialization error: {}", e))
}