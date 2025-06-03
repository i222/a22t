use crate::tasks::processor::TaskProcessor;
use crate::tasks::types::TaskInput;
use tauri::State;

#[tauri::command]
pub async fn cid_run_task(
    task: TaskInput,
    processor: State<'_, TaskProcessor>,
) -> Result<String, String> {
    println!("[cid_run_task] received task: {:?}", task);
    let res = processor.run_task(task).await;
    println!("[cid_run_task] result: {:?}", res);
    res
}

#[tauri::command]
pub async fn cid_abort_task(
    taskId: String,
    processor: State<'_, TaskProcessor>,
) -> Result<(), String> {
    println!("[cid_abort_task] aborting task with id: {}", taskId);
    processor.abort_task(&taskId).await;
    println!("[cid_abort_task] abort requested for task id: {}", taskId);
    Ok(())
}
