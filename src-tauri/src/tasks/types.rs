// // // tauri/src-tauri/src/tasks/types.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    // Single tasks
    TID_ANALYZE_MEDIA_INFO,
    TID_ADD_MEDIAFILE,
    TID_DELETE_MEDIAFILES,
    TID_UPDATE_MEDIAFILE,
    TID_GET_MEDIAFILES_REQ,
    TID_APP_SETTINGS_GET_REQ,
    TID_APP_SETTINGS_CHANGE_REQ,
    TID_APP_SETTINGS_CHANGE_DIR_REQ,

    // Batch tasks
    BTID_DOWNLOAD_MEDIAFILES_REQ,
    BTID_BATCH_TASKS_STATE_PUSH_ON,
    BTID_BATCH_TASKS_STATE_PUSH_OFF,
}

impl TaskType {
    pub fn is_single(&self) -> bool {
        matches!(
            self,
            TaskType::TID_ANALYZE_MEDIA_INFO
                | TaskType::TID_ADD_MEDIAFILE
                | TaskType::TID_DELETE_MEDIAFILES
                | TaskType::TID_UPDATE_MEDIAFILE
                | TaskType::TID_GET_MEDIAFILES_REQ
                | TaskType::TID_APP_SETTINGS_GET_REQ
                | TaskType::TID_APP_SETTINGS_CHANGE_REQ
                | TaskType::TID_APP_SETTINGS_CHANGE_DIR_REQ
        )
    }

    pub fn is_batch(&self) -> bool {
        matches!(
            self,
            TaskType::BTID_DOWNLOAD_MEDIAFILES_REQ
                | TaskType::BTID_BATCH_TASKS_STATE_PUSH_ON
                | TaskType::BTID_BATCH_TASKS_STATE_PUSH_OFF
        )
    }
}

// Для удобства парсинга из строки
impl FromStr for TaskType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TID_ANALYZE_MEDIA_INFO" => Ok(TaskType::TID_ANALYZE_MEDIA_INFO),
            "TID_ADD_MEDIAFILE" => Ok(TaskType::TID_ADD_MEDIAFILE),
            "TID_DELETE_MEDIAFILES" => Ok(TaskType::TID_DELETE_MEDIAFILES),
            "TID_UPDATE_MEDIAFILE" => Ok(TaskType::TID_UPDATE_MEDIAFILE),
            "TID_GET_MEDIAFILES_REQ" => Ok(TaskType::TID_GET_MEDIAFILES_REQ),
            "TID_APP_SETTINGS_GET_REQ" => Ok(TaskType::TID_APP_SETTINGS_GET_REQ),
            "TID_APP_SETTINGS_CHANGE_REQ" => Ok(TaskType::TID_APP_SETTINGS_CHANGE_REQ),
            "TID_APP_SETTINGS_CHANGE_DIR_REQ" => Ok(TaskType::TID_APP_SETTINGS_CHANGE_DIR_REQ),

            "BTID_DOWNLOAD_MEDIAFILES_REQ" => Ok(TaskType::BTID_DOWNLOAD_MEDIAFILES_REQ),
            "BTID_BATCH_TASKS_STATE_PUSH_ON" => Ok(TaskType::BTID_BATCH_TASKS_STATE_PUSH_ON),
            "BTID_BATCH_TASKS_STATE_PUSH_OFF" => Ok(TaskType::BTID_BATCH_TASKS_STATE_PUSH_OFF),

            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub task_type: TaskType,
    pub payload: Value, // payload — общий JSON, парсим вручную в конкретных задачах
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum TaskEvent {
    Progress {
        task_id: String,
        message: Option<String>,
        payload: Option<Value>,
    },
    Result {
        task_id: String,
        message: Option<String>,
        payload: Value,
    },
    Error {
        task_id: String,
        message: Option<String>,
        payload: Option<Value>,
    },
    Cancelled {
        task_id: String,
        message: Option<String>,
        payload: Option<Value>,
    },
    // ADD Broadcast or other event types
}
