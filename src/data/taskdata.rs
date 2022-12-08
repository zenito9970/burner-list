use crate::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskData {
    id: Uuid,
    pub rank: TaskRank,
    pub value: String,
}

impl TaskData {
    pub fn new(rank: TaskRank, value: &str) -> Self {
        TaskData {
            id: Uuid::new_v4(),
            rank,
            value: value.to_owned(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl PartialEq for TaskData {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
