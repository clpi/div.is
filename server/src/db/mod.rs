pub mod sqlite;

// TODO figure out what the hell is going on with postgres (i don't
// think it's in-code setup, likely my enviroment?) or return to
// sqlite until you do
use sqlx::*;
use std::{path::Path, fs};
use crate::models::user::User;

pub use sqlite::Db;
