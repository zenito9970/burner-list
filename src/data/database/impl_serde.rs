use crate::data::*;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};

impl Serialize for DataBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tasks = Vec::<&TaskData>::new();
        for rank in [TaskRank::Primary, TaskRank::Secondary, TaskRank::Other] {
            for task in self.get_by_rank(rank) {
                tasks.push(task);
            }
        }

        let mut s = serializer.serialize_struct("DataBase", 1)?;
        s.serialize_field("tasks", &tasks)?;
        s.end()
    }
}

enum DataBaseField {
    Tasks,
}

struct DataBaseFieldVisitor;
impl<'de> Visitor<'de> for DataBaseFieldVisitor {
    type Value = DataBaseField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("`tasks`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "tasks" => Ok(DataBaseField::Tasks),
            _ => Err(serde::de::Error::unknown_field(v, &["tasks"])),
        }
    }
}

impl<'de> Deserialize<'de> for DataBaseField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_identifier(DataBaseFieldVisitor)
    }
}

struct DataBaseVisitor;
impl<'de> Visitor<'de> for DataBaseVisitor {
    type Value = DataBase;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct DataBase")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut db = DataBase::default();

        while let Some(key) = map.next_key::<DataBaseField>()? {
            match key {
                DataBaseField::Tasks => {
                    let tasks = map.next_value::<Vec<TaskData>>()?;
                    for task in tasks {
                        db.add(task);
                    }
                }
            }
        }

        Ok(db)
    }
}

impl<'de> Deserialize<'de> for DataBase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("DataBase", &["tasks"], DataBaseVisitor)
    }
}
