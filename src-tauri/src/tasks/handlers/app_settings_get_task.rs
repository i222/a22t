// src/tasks/handlers/app_settings_get_task.rs
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
}

pub fn app_settings_get_task(
    store: Arc<Store<Wry<EventLoopMessage>>>,
) -> Arc<dyn Fn(TaskInput, TaskContext) -> BoxFuture<'static, ()> + Send + Sync> {
    Arc::new(move |_input: TaskInput, ctx: TaskContext| {
        let store = store.clone();
        Box::pin(async move {
            (ctx.emit)(TaskEvent::Progress {
                task_id: "app_settings_get".to_string(),
                message: Some("Fetching app settings...".to_string()),
                payload: None,
            });

            let settings: Option<AppSettings> = store.get("appSettings").and_then(|val| {
							if val.is_null() {
									None
							} else {
									serde_json::from_value(val).ok()
							}
					});
					

            let result = settings.unwrap_or_default();

            (ctx.emit)(TaskEvent::Result {
                task_id: "app_settings_get".to_string(),
                message: Some("Fetched settings".to_string()),
                payload: serde_json::to_value(&result).unwrap(),
            });
        })
    })
}
