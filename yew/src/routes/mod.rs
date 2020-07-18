use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod about;
pub mod home;
pub mod contact;
pub mod dash;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/dash"]
    Dash,
    #[to = "/contact"]
    Contact,
    #[to = "/about"]
    About,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Home,
}
