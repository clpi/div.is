use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::models::user::User;

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user: Option<User>,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="navbar-brand">
                        { "div.is" }
                    </RouterAnchor<AppRoute>>
                    {
                        if let Some(user) = &self.props.user {
                            self.logged_in(&user)
                        } else {
                            self.not_logged_in()
                        }
                    }
                </div>
            </nav>
        }
    }
}

impl Header {
    fn not_logged_in(&self) -> Html {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="app-link">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::About classes="app-link">
                        { "About" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Contact classes="app-link">
                        { "Contact" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="app-link">
                    <RouterAnchor<AppRoute> route=AppRoute::Login classes="nav-link">
                        { "Sign in" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="app-link">
                    <RouterAnchor<AppRoute> route=AppRoute::Signup classes="nav-link">
                        { "Sign up" }
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>
        }
    }

    fn logged_in(&self, user: &User) -> Html {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="app-link">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="app-link">
                    <RouterAnchor<AppRoute> route=AppRoute::Records classes="nav-link">
                        { "Records" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="app-link">
                    <RouterAnchor<AppRoute> route=AppRoute::Profile  classes="nav-link">
                        { &user.username }
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>
        }
    }
}
