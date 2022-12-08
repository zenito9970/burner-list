use crate::prelude::*;
use uuid::Uuid;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct DraggingCardId {
    pub id: Option<Uuid>,
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct DraggingPositionIndex {
    pub index: Option<usize>,
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct DraggingPositionRank {
    pub rank: Option<TaskRank>,
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct DraggingInitialPos {
    pub pos: Option<(i32, i32)>,
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct DraggingMousePos {
    pub pos: Option<(i32, i32)>,
}
