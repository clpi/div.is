use serde_derive::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use super::Db;
use sqlx::{sqlite::*, Sqlite, types::chrono::{DateTime, Utc}};
use crate::api::auth::hash_pwd;

//TODO Consider adding custom types for forieng key references, using
//TODO Severely refactor code to reduce redundancy (query builder?)
//TODO Consider removing redundancy between EntryType/Item
//TODO Separate implementations into own files
//TODO error handling for with_x functions for models w/o ids -- remove unwrap()
//sqlx::Type and transparent
//NOTE: Add (?) last login, pwd hash
//TODO make password option??
//TODO only make async what needs to be async
//TODO split up models into src/models in appropriate files
// TODO: Eventually make records collaborable, not associated with single user
// TODO: make collaborative records have anonymous option
//#[derive(Serialize, Deserialize)]









// a label is a Field
// a task is an item
// a goal is an item



//TODO allow for requests like User::with_id(id).delete() or User::with_username(









pub enum RecordPrivelege {
    
}

/*
 *
 * Vague ideas:
 * Factoid objects: can interplay with rules, conditions, etc. to provide a portrait over time
 * and provide insights into other entries
*/
// NOTE Right now, entries must contain items in order to contain their fields -- they can choose
// fields individually from items, but cannot make a ifeld not related to an item
