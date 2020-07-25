pub mod auth;

use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

pub struct Request<T, U> where
    T: Into<String>,
    U: Deserialize + 'static, 
{
    method: String,
    url: String,
    body: T,
    callback: Callback<Result<U, Error>>,
}

impl Request<T, U> {

}

pub struct RequestBuilder {

}
