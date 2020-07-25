use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod about;
pub mod home;
pub mod contact;
pub mod dash;
pub mod signup;
pub mod login;
pub mod profile;
pub mod records;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/dash"]
    Dash,
    #[to = "/login"]
    Login,
    #[to = "/signup"]
    Signup,
    #[to = "/records"]
    Records,
    #[to = "/contact"]
    Contact,
    #[to = "/about"]
    About,
    #[to = "/profile"] //should be /:username
    Profile,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Home,
}
