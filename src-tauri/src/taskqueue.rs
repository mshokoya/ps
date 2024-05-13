// // #[macro_use]
// // extern crate lazy_static;
// use crate::defaults::{ActionTaskGroup, ActionTaskType, TaskQueueTaskGroup, TaskQueueTaskType};
// use lazy_static::lazy_static;
// use std::collections::VecDeque;
// use std::io::Error;
// use std::time::Duration;
// use std::{collections::HashMap, sync::Mutex};
// use tauri::AppHandle;
// use tokio::spawn;
// use tokio::time::sleep;
// use uuid::Uuid;

// lazy_static! {
//     pub static ref WAIT_QUEUE: Mutex<VecDeque<Task>> = Mutex::new(VecDeque::new());
//     pub static ref PROCESS_QUEUE: Mutex<VecDeque<Process>> = Mutex::new(VecDeque::new());
//     pub static ref TIMEOUT_QUEUE: Mutex<VecDeque<Task>> = Mutex::new(VecDeque::new());
//     pub static ref MAX_PS: Mutex<MaxPS> = Mutex::new(MaxPS::new(5));
// }

// pub struct MaxPS {
//     ps: usize,
// }

// impl MaxPS {
//     pub fn new(val: usize) -> Self {
//         MaxPS { ps: val }
//     }
//     pub fn get(&self) -> usize {
//         self.ps
//     }
//     pub fn set(&mut self, val: usize) {
//         self.ps = val
//     }
// }

// pub struct ActionData {
//     task_type: ActionTaskType,
//     task_group: ActionTaskGroup,
//     metadata: HashMap<String, String>,
// }

// #[derive(Clone, Copy)]
// pub struct TQTimeout {
//     time: u64,
//     rounds: u8,
//     // _exec: Timeout<&'static str>,
// }

// pub struct Task {
//     task_id: Uuid,
//     task_group: TaskQueueTaskGroup,
//     task_type: TaskQueueTaskType,
//     message: &'static str,
//     timeout: Option<TQTimeout>,
//     metadata: HashMap<String, String>,
//     action_data: ActionData,
//     action: ActionTaskType,
// }

// pub struct Process {
//     task: Task,
//     p: &'static str,
// }

// pub struct TaskQueue {
//     app: AppHandle,
// }

// // https://stackoverflow.com/questions/54971024/accessing-a-method-of-self-inside-a-thread-in-rust
// // https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
// impl TaskQueue {
//     pub fn new(app: AppHandle) -> TaskQueue {
//         TaskQueue { app }
//     }

//     pub fn w_enqueue(task: Task) {
//         w_enqueue(task)
//     }
//     pub fn w_dequeue() -> Option<Task> {
//         w_dequeue()
//     }
//     pub fn p_enqueue(p: Process) {
//         p_enqueue(p)
//     }
//     pub fn p_dequeue(task_id: Uuid) {
//         p_dequeue(task_id)
//     }
//     pub fn t_enqueue(task: Task) {
//         t_enqueue(task)
//     }
//     pub fn t_dequeue(task_id: Uuid) {
//         t_dequeue(task_id)
//     }
// }

// fn w_enqueue(task: Task) {
//     WAIT_QUEUE.lock().unwrap().push_back(task)
// }

// fn w_dequeue() -> Option<Task> {
//     WAIT_QUEUE.lock().unwrap().pop_front()
// }

// fn p_enqueue(process: Process) {
//     PROCESS_QUEUE.lock().unwrap().push_back(process)
// }

// fn p_dequeue(task_id: Uuid) {
//     let process = remove_process_task(&task_id);

//     if let Some(ps) = process {
//         if let Some(mut t_out) = ps.task.timeout {
//             t_out.rounds -= 1;
//             if t_out.rounds > 0 {
//                 t_enqueue(ps.task);
//             } else {
//                 t_dequeue(task_id);
//             }
//         }
//     }
// }

// fn t_enqueue(task: Task) {
//     let process = remove_process_task(&task.task_id);
//     if let Some(ps) = process {
//         let dur = ps.task.timeout.unwrap().time.clone();
//         spawn(async move {
//             sleep(Duration::from_millis(dur)).await;
//             w_enqueue(ps.task);
//         });
//     }
// }

// fn t_dequeue(task_id: Uuid) {
//     let mut pq = PROCESS_QUEUE.lock().unwrap();
//     let idx = pq
//         .iter()
//         .position(|t| t.task.task_id == task_id)
//         .unwrap_or(225);

//     pq.swap_remove_back(idx);
// }

// fn exec() -> Result<(), Error> {
//     let max_ps: usize = MAX_PS.lock().unwrap().get();
//     let ps_len: usize = PROCESS_QUEUE.lock().unwrap().len();

//     if ps_len >= max_ps {
//         return Ok(());
//     };

//     let task = match w_dequeue() {
//         None => return Ok(()),
//         Some(tsk) => tsk,
//     };

//     // let process = spawn(async)

//     Ok(())
// }

// fn remove_timeout_task(task_id: &Uuid) -> Option<Task> {
//     let mut tq = TIMEOUT_QUEUE.lock().unwrap();
//     let idx = tq.iter().position(|t| t.task_id == *task_id).unwrap_or(225);
//     tq.swap_remove_back(idx)
// }

// fn remove_wait_task(task_id: &Uuid) -> Option<Task> {
//     let mut wq = WAIT_QUEUE.lock().unwrap();
//     let idx = wq.iter().position(|t| t.task_id == *task_id).unwrap_or(225);
//     wq.swap_remove_back(idx)
// }

// fn remove_process_task(task_id: &Uuid) -> Option<Process> {
//     let mut pq = PROCESS_QUEUE.lock().unwrap();
//     let idx = pq
//         .iter()
//         .position(|t| t.task.task_id == *task_id)
//         .unwrap_or(225);
//     pq.swap_remove_back(idx)
// }

// // =====================================
// // ====================================
// // ====================================

// // #[macro_use]
// // extern crate lazy_static;
// use crate::defaults::{ActionTaskGroup, ActionTaskType, TaskQueueTaskGroup, TaskQueueTaskType};
// use lazy_static::lazy_static;
// use std::collections::VecDeque;
// use std::io::Error;
// use std::sync::Arc;
// use std::time::Duration;
// use std::{collections::HashMap, sync::Mutex};
// use tauri::{App, AppHandle, Manager, Window};
// use tokio::spawn;
// use tokio::time::sleep;
// use uuid::Uuid;

// // lazy_static! {
// //     pub static ref TQ: TaskQueue = TaskQueue::new();
// // }

// struct ActionData {
//     task_type: ActionTaskType,
//     task_group: ActionTaskGroup,
//     metadata: HashMap<String, String>,
// }

// #[derive(Clone, Copy)]
// struct TQTimeout {
//     time: u64,
//     rounds: u8,
//     // _exec: Timeout<&'static str>,
// }

// struct Task {
//     task_id: Uuid,
//     task_group: TaskQueueTaskGroup,
//     task_type: TaskQueueTaskType,
//     message: &'static str,
//     timeout: Option<TQTimeout>,
//     metadata: HashMap<String, String>,
//     action_data: ActionData,
//     action: ActionTaskType,
// }

// struct Process {
//     task: Task,
//     p: &'static str,
// }

// pub struct TaskQueue {
//     app: AppHandle,
//     pub wait_queue: Mutex<VecDeque<Task>>,
//     process_queue: Mutex<VecDeque<Process>>,
//     timeout_queue: Mutex<VecDeque<Task>>,
//     max_processes: u8,
// }

// unsafe impl Send for TaskQueue {}
// unsafe impl Sync for TaskQueue {}
// // https://stackoverflow.com/questions/54971024/accessing-a-method-of-self-inside-a-thread-in-rust
// // https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
// impl TaskQueue {
//     pub fn new(app: AppHandle) -> Self {
//         Self {
//             app,
//             wait_queue: Mutex::new(VecDeque::new()),
//             process_queue: Mutex::new(VecDeque::new()),
//             timeout_queue: Mutex::new(VecDeque::new()),
//             max_processes: 5,
//         }
//     }

//     pub fn w_enqueue(&self, task: Task) {
//         self.wait_queue.lock().unwrap().push_back(task)
//     }

//     fn w_dequeue(&self) -> Option<Task> {
//         self.wait_queue.lock().unwrap().pop_front()
//     }

//     fn p_enqueue(&self, process: Process) {
//         self.process_queue.lock().unwrap().push_back(process)
//     }

//     fn p_dequeue(&self, task_id: Uuid) {
//         let process = remove_process(&self.process_queue, &task_id);

//         if let Some(ps) = process {
//             if let Some(mut t_out) = ps.task.timeout {
//                 t_out.rounds -= 1;
//                 if t_out.rounds > 0 {
//                     self.t_enqueue(ps.task);
//                 } else {
//                     self.t_dequeue(task_id);
//                 }
//             }
//         }
//     }

//     fn t_enqueue(&self, task: Task) {
//         let task = remove_task(&self.timeout_queue, &task.task_id);

//         if let Some(tsk) = task {
//             if let Some(t_out) = tsk.timeout {
//                 let s = self.app.app_handle().clone();
//                 tauri::async_runtime::spawn(async move {
//                     sleep(Duration::from_millis(t_out.time.clone())).await;
//                     // self.app.state::<TaskQueue>().w_enqueue(tsk);
//                     s.state::<TaskQueue>().w_enqueue(tsk);
//                     //
//                     // self.w_enqueue(tsk);
//                 });
//             }
//         }
//     }

//     fn t_dequeue(&self, task_id: Uuid) -> Option<Process> {
//         let mut pq = self.process_queue.lock().unwrap();
//         let idx = pq
//             .iter()
//             .position(|t| t.task.task_id == task_id)
//             .unwrap_or(225);

//         pq.swap_remove_back(idx)
//     }

//     fn exec(&self) -> Result<(), Error> {
//         if self.process_queue.lock().unwrap().len() >= self.max_processes.into() {
//             return Ok(());
//         };

//         let task = match self.w_dequeue() {
//             None => return Ok(()),
//             Some(tsk) => tsk,
//         };

//         let process = return Ok(());
//     }
// }

// fn remove_task(queue: &Mutex<VecDeque<Task>>, task_id: &Uuid) -> Option<Task> {
//     let mut tq = queue.lock().unwrap();
//     let idx = tq.iter().position(|t| t.task_id == *task_id).unwrap_or(225);
//     tq.swap_remove_back(idx)
// }

// fn remove_process(queue: &Mutex<VecDeque<Process>>, task_id: &Uuid) -> Option<Process> {
//     let mut pq = queue.lock().unwrap();
//     let idx = pq
//         .iter()
//         .position(|t| t.task.task_id == *task_id)
//         .unwrap_or(225);
//     pq.swap_remove_back(idx)
// }

// =====================================
// ====================================
// ====================================

// #[macro_use]
// extern crate lazy_static;
use crate::defaults::{ActionData, Channels, TaskArgs, TaskEvent, TaskGroup, TaskType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;
use std::io::Error;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, sync::Mutex};
use tauri::{AppHandle, Manager};
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct TQTimeout {
    pub time: u64,
    pub rounds: u8,
    // _exec: Timeout<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub task_id: Uuid,
    pub task_group: TaskGroup,
    pub task_type: TaskType,
    pub message: &'static str,
    pub timeout: Option<TQTimeout>,
    pub metadata: Option<Arc<HashMap<String, Value>>>,
    pub args: Option<Arc<TaskArgs>>,
}

#[derive(Debug)]
pub struct Process {
    pub task: Task,
    pub ps: JoinHandle<()>,
}

#[derive(Debug)]
pub struct TaskQueue {
    pub app_handle: AppHandle,
    pub wait_queue: Mutex<VecDeque<Task>>,
    pub process_queue: Mutex<VecDeque<Process>>,
    pub timeout_queue: Mutex<VecDeque<Task>>,
    pub max_processes: u8,
}

unsafe impl Send for TaskQueue {}
unsafe impl Sync for TaskQueue {}
// https://stackoverflow.com/questions/54971024/accessing-a-method-of-self-inside-a-thread-in-rust
// https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
impl TaskQueue {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            wait_queue: Mutex::new(VecDeque::new()),
            process_queue: Mutex::new(VecDeque::new()),
            timeout_queue: Mutex::new(VecDeque::new()),
            max_processes: 2,
        }
    }

    pub fn w_enqueue(&self, task: Task) {
        println!("WE IN DA TASK QUEUE");
        let task_cln = task.clone();
        self.wait_queue.lock().unwrap().push_back(task_cln);
        self.app_handle
            .emit(
                Channels::WaitQueue.into(),
                TaskEvent {
                    task_id: task.task_id,
                    message: "added new task to wait queue",
                    task_type: TaskType::Enqueue,
                    metadata: task.metadata,
                    action_data: ActionData {
                        task_group: task.task_group,
                        task_type: task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
        self.exec();
    }

    fn w_dequeue(&self) -> Option<Task> {
        let task = match self.wait_queue.lock().unwrap().pop_front() {
            None => return None,
            Some(tsk) => tsk,
        };
        let task_cln = task.clone();
        self.app_handle
            .emit(
                Channels::WaitQueue.into(),
                TaskEvent {
                    task_id: task_cln.task_id,
                    message: "removed task from wait queue",
                    task_type: TaskType::Dequeue,
                    metadata: task_cln.metadata,
                    action_data: ActionData {
                        task_group: task_cln.task_group,
                        task_type: task_cln.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
        Some(task)
    }

    fn p_enqueue(&self, process: Process) {
        let task = process.task.clone();
        self.process_queue.lock().unwrap().push_back(process);
        self.app_handle
            .emit(
                Channels::ProcessQueue.into(),
                TaskEvent {
                    task_id: task.task_id,
                    message: "new task added to processing queue",
                    task_type: TaskType::Enqueue,
                    metadata: task.metadata,
                    action_data: ActionData {
                        task_group: task.task_group,
                        task_type: task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
        self.exec();
    }

    fn p_dequeue(&self, task_id: Uuid) {
        let ps = remove_process(&self.process_queue, &task_id).unwrap();
        let task = ps.task.clone();

        if let Some(mut t_out) = ps.task.timeout {
            t_out.rounds -= 1;
            if t_out.rounds > 0 {
                self.t_enqueue(task);
            } else {
                self.t_dequeue(task_id);
            }
        }

        self.app_handle
            .emit(
                Channels::ProcessQueue.into(),
                TaskEvent {
                    task_id,
                    message: "removed completed task from queue",
                    task_type: TaskType::Dequeue,
                    metadata: ps.task.metadata,
                    action_data: ActionData {
                        task_group: ps.task.task_group,
                        task_type: ps.task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
    }

    fn t_enqueue(&self, task: Task) {
        let task = remove_task(&self.timeout_queue, &task.task_id).unwrap();

        let task_cln = task.clone();
        if let Some(timeout) = task_cln.timeout {
            let app_handle = self.app_handle.clone();
            spawn(async move {
                sleep(Duration::from_millis(timeout.time.clone())).await;
                app_handle.state::<TaskQueue>().w_enqueue(task_cln);
            });
        }

        self.app_handle
            .emit(
                Channels::TimeoutQueue.into(),
                TaskEvent {
                    task_id: task.task_id,
                    message: "removed completed task from queue",
                    task_type: TaskType::Enqueue,
                    metadata: task.metadata,
                    action_data: ActionData {
                        task_group: task.task_group,
                        task_type: task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();

        self.exec();
    }

    fn t_dequeue(&self, task_id: Uuid) {
        let mut pq = self.process_queue.lock().unwrap();
        let idx = pq
            .iter()
            .position(|t| t.task.task_id == task_id)
            .unwrap_or(225);

        let task = pq.swap_remove_back(idx).unwrap().task;

        self.app_handle
            .emit(
                Channels::TimeoutQueue.into(),
                TaskEvent {
                    task_id: task.task_id,
                    message: "removed completed task from queue",
                    task_type: TaskType::Dequeue,
                    metadata: task.metadata,
                    action_data: ActionData {
                        task_group: task.task_group,
                        task_type: task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
    }

    fn exec(&self) {
        let pq = self.process_queue.lock().unwrap();
        let pq_len = pq.len();

        if pq_len >= self.max_processes.into() {
            return;
        };
        drop(pq);

        let task = match self.w_dequeue() {
            None => return,
            Some(tsk) => tsk,
        };
        let app_handle = self.app_handle.clone();
        let tsk_cln = task.clone();

        let ps = spawn(async move {
            println!("IN DA SPAWN");

            app_handle
                .emit(
                    Channels::ProcessQueue.into(),
                    TaskEvent {
                        task_id: tsk_cln.task_id,
                        message: "removed completed task from queue",
                        task_type: TaskType::Enqueue,
                        metadata: tsk_cln.metadata,
                        action_data: ActionData {
                            task_group: tsk_cln.task_group,
                            task_type: tsk_cln.task_type,
                            metadata: None,
                        },
                    },
                )
                .unwrap();
        });

        let mut pq = self.process_queue.lock().unwrap();
        pq.push_back(Process { task, ps });
        drop(pq);
        // self.exec();
    }
}

fn remove_task(queue: &Mutex<VecDeque<Task>>, task_id: &Uuid) -> Option<Task> {
    let mut tq = queue.lock().unwrap();
    let idx = tq.iter().position(|t| t.task_id == *task_id).unwrap_or(225);
    tq.swap_remove_back(idx)
}

fn remove_process(queue: &Mutex<VecDeque<Process>>, task_id: &Uuid) -> Option<Process> {
    let mut pq = queue.lock().unwrap();
    let idx = pq
        .iter()
        .position(|t| t.task.task_id == *task_id)
        .unwrap_or(225);
    pq.swap_remove_back(idx)
}
