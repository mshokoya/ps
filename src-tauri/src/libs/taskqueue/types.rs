use async_std::task::JoinHandle;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use uuid::Uuid;

use crate::actions::controllers::TaskType;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct TQTimeout {
    pub time: u64,
    pub rounds: u8,
    // _exec: Option<Arc<Sleep>>,
}

#[derive(Clone, Debug)]
pub struct Task {
    // pub task_id: Uuid,
    pub task_id: String,
    pub task_group: TaskGroup,
    pub task_type: TaskType,
    pub message: &'static str,
    pub timeout: Option<TQTimeout>,
    pub metadata: Option<Value>,
    pub args: Option<Value>,
}

#[derive(Debug)]
pub struct Process {
    pub task: Task,
    pub ps: JoinHandle<()>,
}

#[derive(Clone, Serialize, Debug)]
pub struct TaskEvent<'a> {
    // pub task_id: &'a Uuid,
    pub task_id: &'a String,
    pub message: String,
    pub ok: Option<bool>,
    pub task_type: TaskType,
    pub metadata: &'a Option<Value>,
    pub action_data: ActionData<'a>,
}

#[derive(Clone, Serialize, Debug)]
pub struct ActionData<'a> {
    pub task_group: &'a TaskGroup,
    pub task_type: &'a TaskType,
    pub metadata: Option<Value>,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum TaskGroup {
    Enqueue,
    Dequeue,
    Apollo,
}

#[derive(Debug)]
pub enum Channels {
    WaitQueue,
    ProcessQueue,
    TimeoutQueue,
    Apollo,
}

impl Into<&str> for Channels {
    fn into(self) -> &'static str {
        match self {
            Channels::WaitQueue => "waitQueue",
            Channels::ProcessQueue => "processQueue",
            Channels::TimeoutQueue => "timeoutQueue",
            Channels::Apollo => "apollo",
        }
    }
}
