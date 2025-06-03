use crate::tasks::processor::{TaskContext, TaskHandler};
use crate::tasks::types::{TaskEvent, TaskInput};
use futures::future::BoxFuture;
use std::sync::Arc;

pub fn analyze_media_info_handler() -> TaskHandler {
    Arc::new(
        |input: TaskInput, ctx: TaskContext| -> BoxFuture<'static, ()> {
            Box::pin(async move {
                // Emit progress event
                (ctx.emit)(TaskEvent::Progress {
                    task_id: "some_id".to_string(),
                    message: Some("Task started".to_string()),
                    payload: Some(input.payload),
                });

                // Simulate work
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                // Emit result event
                (ctx.emit)(TaskEvent::Result {
                    task_id: "some_id".to_string(),
                    message: Some("Task finished".to_string()),
                    payload: serde_json::json!({ "status": "ok" }),
                });
            })
        },
    )
}
