mod impl_serde;

use crate::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use im::{HashMap, Vector};
use slab::Slab;
use uuid::Uuid;
use yewdux::prelude::*;

#[derive(Store, Clone)]
pub struct DataBase {
    store: Slab<TaskData>,
    id_index: HashMap<Uuid, usize>,
    rank_index: HashMap<TaskRank, Vector<usize>>,
    hash: Uuid,
}

impl Default for DataBase {
    fn default() -> Self {
        Self {
            store: Slab::default(),
            id_index: HashMap::default(),
            rank_index: HashMap::default(),
            hash: Uuid::new_v4(),
        }
    }
}

impl PartialEq for DataBase {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl core::fmt::Debug for DataBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let store = self
            .store
            .iter()
            .map(move |(i, task)| format!("({}, {:?})", i, task))
            .collect::<Vec<_>>();
        f.debug_struct("DataBase")
            .field("hash", &self.hash)
            .field("_store", &store)
            .finish()
    }
}

impl DataBase {
    pub fn load_from_local_storage() -> Self {
        if let Ok(db) = LocalStorage::get::<DataBase>("tasks") {
            db
        } else {
            let mut db = Self::default();

            // dummy
            db.add(TaskData::new(
                TaskRank::Primary,
                "Hello! BurnerList is a simple, intentionally constrained list.",
            ));

            db
        }
    }

    pub fn apply_event(&mut self, event: &TaskEvent) -> bool {
        log::debug!("[database] {:?}", event);
        // log::debug!("[database] {:#?}", self);

        match event {
            TaskEvent::Add(TaskAddData { rank, value }) => {
                let task = TaskData::new(*rank, &value);
                self.add(task);

                log::debug!("[database] {:#?}", self);
                return true;
            }
            TaskEvent::Edit(TaskEditData { id, value }) => {
                if let Some((index, mut task)) = self.remove_by_id(*id) {
                    task.value = value.clone();
                    self.insert(task, Some(index));

                    log::debug!("[database] {:#?}", self);
                    return true;
                }
            }
            &TaskEvent::Move(TaskMoveData { id, rank, index }) => {
                if let Some((_, mut task)) = self.remove_by_id(id) {
                    task.rank = rank;
                    self.insert(task, index);

                    log::debug!("[database] {:#?}", self);
                    return true;
                }
            }
            &TaskEvent::Delete(TaskDeleteData { id }) => {
                if let Some(_) = self.remove_by_id(id) {
                    log::debug!("[database] {:#?}", self);
                    return true;
                }
            }
        }

        false
    }

    pub fn burn_tasks(&mut self, rank: TaskRank) {
        let task_ids = self
            .get_by_rank(rank)
            .iter()
            .map(move |task| task.id())
            .collect::<Vec<_>>();
        for id in task_ids {
            self.remove_by_id(id);
        }
    }

    pub fn swap_tasks(&mut self, rank1: TaskRank, rank2: TaskRank) {
        let mut primary_tasks = vec![];
        let primary_task_ids = self
            .get_by_rank(rank1)
            .iter()
            .map(move |task| task.id())
            .collect::<Vec<_>>();
        for id in primary_task_ids {
            if let Some((_, task)) = self.remove_by_id(id) {
                primary_tasks.push(task);
            }
        }

        let mut secondary_tasks = vec![];
        let secondary_task_ids = self
            .get_by_rank(rank2)
            .iter()
            .map(move |task| task.id())
            .collect::<Vec<_>>();
        for id in secondary_task_ids {
            if let Some((_, task)) = self.remove_by_id(id) {
                secondary_tasks.push(task);
            }
        }

        for mut task in primary_tasks {
            task.rank = rank2;
            self.add(task);
        }

        for mut task in secondary_tasks {
            task.rank = rank1;
            self.add(task);
        }
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&TaskData> {
        Some(&self.store[*self.id_index.get(&id)?])
    }

    pub fn get_by_rank(&self, rank: TaskRank) -> Vec<&TaskData> {
        if let Some(set) = self.rank_index.get(&rank) {
            set.iter().map(move |&i| &self.store[i]).collect()
        } else {
            vec![]
        }
    }

    fn add(&mut self, task: TaskData) {
        let i = self.store.insert(task);
        let task = &self.store[i];
        self.id_index.insert(task.id(), i);
        self.rank_index
            .entry(task.rank)
            .or_insert(Vector::new())
            .push_back(i);
        self.hash = Uuid::new_v4();
    }

    fn insert(&mut self, task: TaskData, index: Option<usize>) {
        let i = self.store.insert(task);
        let task = &self.store[i];
        self.id_index.insert(task.id(), i);

        let v = self.rank_index.entry(task.rank).or_insert(Vector::new());
        if let Some(idx) = index {
            v.insert(idx, i);
        } else {
            v.push_back(i);
        }

        self.hash = Uuid::new_v4();
    }

    fn remove_by_id(&mut self, id: Uuid) -> Option<(usize, TaskData)> {
        let i = self.id_index.remove(&id)?;
        let task = self.store.remove(i);
        self.id_index.remove(&task.id());
        if let Some(set) = self.rank_index.get_mut(&task.rank) {
            if let Some(i) = set.index_of(&i) {
                set.remove(i);
                self.hash = Uuid::new_v4();
                return Some((i, task));
            }
        }

        None
    }
}
