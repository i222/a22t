// // tauri/src-tauri/src/tasks/processor.rs

use futures::future::{AbortHandle, Abortable};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::tasks::types::{cid, TaskEvent, TaskInput, TaskType};

pub type TaskHandler =
    Arc<dyn Fn(TaskInput, TaskContext) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct TaskContext {
    pub emit: Box<dyn Fn(TaskEvent) + Send + Sync>,
    pub abort_handle: AbortHandle,
    pub task_id: String,
}

pub struct TaskProcessor {
    handlers: HashMap<TaskType, TaskHandler>,
    active_tasks: Mutex<HashMap<String, AbortHandle>>,
    app_handle: AppHandle,
}

impl TaskProcessor {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            handlers: HashMap::new(),
            active_tasks: Mutex::new(HashMap::new()),
            app_handle,
        }
    }

    pub fn register_handler(&mut self, task_type: TaskType, handler: TaskHandler) {
        if self.handlers.contains_key(&task_type) {
            panic!("Handler already registered for {:?}", task_type);
        }
        self.handlers.insert(task_type, handler);
    }

    pub async fn run_task(&self, input: TaskInput) -> Result<String, String> {
        let task_id = Uuid::new_v4().to_string();
        let task_id_for_emit = task_id.clone();

        let handler = self
            .handlers
            .get(&input.task_type)
            .ok_or_else(|| format!("No handler registered for {:?}", input.task_type))?;

        let (abort_handle, abort_registration) = AbortHandle::new_pair();

        self.active_tasks
            .lock()
            .await
            .insert(task_id.clone(), abort_handle.clone());

        // let emit = move |event: TaskEvent| {
        //     println!("Emit event for task {}: {:?}", task_id_for_emit, event);
        // };
        let app_handle = self.app_handle.clone();
        let emit = move |event: TaskEvent| {
            println!("[Proc][Emit]: {:?}", event);
            let app = app_handle.clone();
            tokio::spawn(async move {
                if let Err(e) = app.emit(cid::ON_TASK_PROCESSOR_EVENT, event) {
                    eprintln!("Failed to emit event: {}", e);
                }
            });
        };

        let ctx = TaskContext {
            emit: Box::new(emit),
            abort_handle: abort_handle.clone(),
            task_id: task_id.clone(),
        };

        let input_clone = input.clone();
        let handler = handler.clone();

        tokio::spawn(async move {
            let fut = handler(input_clone, ctx);
            let abortable = Abortable::new(fut, abort_registration);
            let _ = abortable.await;
        });

        Ok(task_id)
    }

    pub async fn abort_task(&self, task_id: &str) {
        if let Some(handle) = self.active_tasks.lock().await.remove(task_id) {
            handle.abort();
        }
    }
}
