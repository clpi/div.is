use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nav {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav>
                <ul class="navbar">
                    <li class="nav"><RouterAnchor<AppRoute> route=AppRoute::Home classes="app-link" >{ "Home" }</RouterAnchor<AppRoute>></li>
                    <li class="nav"><RouterAnchor<AppRoute> route=AppRoute::About classes="app-link">{ "About" }</RouterAnchor<AppRoute>></li>
                    <li class="nav"><RouterAnchor<AppRoute> route=AppRoute::Contact classes="app-link">{ "Contact" }</RouterAnchor<AppRoute>></li>
                    <li class="nav"><RouterAnchor<AppRoute> route=AppRoute::Login classes="app-link">{ "Login" }</RouterAnchor<AppRoute>></li>
                    <li class="nav"><RouterAnchor<AppRoute> route=AppRoute::Signup classes="app-link">{ "Signup" }</RouterAnchor<AppRoute>></li>
                </ul>
            </nav>
        }
    }
}
