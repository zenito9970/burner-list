use crate::prelude::*;
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub enum TaskEvent {
    Add(TaskAddData),
    Edit(TaskEditData),
    Move(TaskMoveData),
    Delete(TaskDeleteData),
}

#[derive(PartialEq, Debug)]
pub struct TaskAddData {
    pub rank: TaskRank,
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct TaskEditData {
    pub id: Uuid,
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct TaskMoveData {
    pub id: Uuid,
    pub rank: TaskRank,
    pub index: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub struct TaskDeleteData {
    pub id: Uuid,
}
