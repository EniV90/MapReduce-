use serde::{Deserialize, Serialize};

use crate::TaskKind;

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskRequest {}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskReply {
    pub task: TaskKind,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DoneRequest {
    pub task: TaskKind,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DoneReply {}
