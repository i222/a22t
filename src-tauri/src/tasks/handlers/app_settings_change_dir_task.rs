// // tauri/src-tauri/src/tasks/app_settings_change_dir_task.rs
use crate::tasks::processor::TaskContext;
use crate::tasks::types::{TaskEvent, TaskInput};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::EventLoopMessage;
use tauri_plugin_store::Store;
use tauri_runtime_wry::Wry;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppSettings {
    pub base_download_dir: String,
    // Other fields if are needed
}

pub fn app_settings_change_dir_task(
    store: Arc<Store<Wry<EventLoopMessage>>>,
) -> Arc<dyn Fn(TaskInput, TaskContext) -> BoxFuture<'static, ()> + Send + Sync> {
    Arc::new(move |input: TaskInput, ctx: TaskContext| {
        let store = store.clone();
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Payload {
                change_field: String,
                new_value: String,
            }

            let payload: Payload = match serde_json::from_value(input.payload) {
                Ok(p) => p,
                Err(e) => {
                    (ctx.emit)(TaskEvent::Error {
                        task_id: "app_settings_change_dir".to_string(),
                        message: Some(format!("Invalid payload: {}", e)),
                        payload: None,
                    });
                    return;
                }
            };

            if payload.change_field != "baseDownloadDir" {
                (ctx.emit)(TaskEvent::Error {
                    task_id: "app_settings_change_dir".to_string(),
                    message: Some(format!(
                        "Unsupported change field: {}",
                        payload.change_field
                    )),
                    payload: None,
                });
                return;
            }

            let settings: AppSettings = match store.get("appSettings").and_then(|val| {
                if val.is_null() {
                    None
                } else {
                    serde_json::from_value(val).ok()
                }
            }) {
                Some(s) => s,
                None => AppSettings::default(),
            };

            let mut new_settings = settings.clone();
            new_settings.base_download_dir = payload.new_value;

            // save new changes
            // if let Err(e) = store.insert("appSettings", serde_json::to_value(&new_settings).unwrap()) {
            //     (ctx.emit)(TaskEvent::Error {
            //         task_id: "app_settings_change_dir".to_string(),
            //         message: Some(format!("Failed to save settings: {}", e)),
            //         payload: None,
            //     });
            //     return;
            // }
            store.set("appSettings", serde_json::to_value(&new_settings).unwrap());

            if let Err(e) = store.save() {
                (ctx.emit)(TaskEvent::Error {
                    task_id: "app_settings_change_dir".to_string(),
                    message: Some(format!("Failed to persist settings: {}", e)),
                    payload: None,
                });
                return;
            }

            if let Err(e) = store.save() {
                (ctx.emit)(TaskEvent::Error {
                    task_id: "app_settings_change_dir".to_string(),
                    message: Some(format!("Failed to persist settings: {}", e)),
                    payload: None,
                });
                return;
            }

            (ctx.emit)(TaskEvent::Result {
                task_id: "app_settings_change_dir".to_string(),
                message: Some("Settings updated".to_string()),
                payload: serde_json::to_value(&new_settings).unwrap(),
            });
        })
    })
}
