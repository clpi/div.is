use sqlx::{Postgres, FromRow, postgres::*};
use crate::db::Db;
use super::{
    Time, Permission, Status, Priority, Model,
    link::FieldEntryLink,
};

// a logic implementation to connect actions in web app
// to entries, rules, fields, etc.
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Rule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub rid: Option<i32>, // record ID
    pub aid: Option<i32>, //action id
    pub name: String,
    #[serde(default = "Priority::lowest")]
    pub priority: String,
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub target: String,
    pub action: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

// NOTE: Define as enum for in-code options TODO allow for tiered conditions
// TODO Create table for actions performed on the basis of conditions
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Condition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub pos: i32, //position in rule
    pub and_or: bool, //whether this is preceded by "and". 0 if preceded by "or"
    pub ruleid: Option<i32>, //can be associated only with a single record
    pub iid1: Option<i32>, //can be associated only with a single item if wished
    pub fid1: i32, //must be associated with a field, whether singular or global
    pub iid2: Option<i32>, 
    pub fid2: i32,
    pub cond: i32, // < <= = >= > etc
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

pub enum Conditions {}
