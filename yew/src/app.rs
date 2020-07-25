use yew::prelude::*;
use yew::Properties;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, route::Route};

use crate::components::nav::Nav;
//use crate::components::header::Header;
use crate::routes::{about::About, home::Home, contact::Contact, dash::Dash,login::Login, signup::Signup, records::Records, profile::Profile, AppRoute};
use crate::models::user::User;

/// Root component
pub struct App{
    current_user: Option<User>,
    //props: Props,
}

pub enum Msg {
    Refresh(User),
    Logout,
    Route(Route),
}

 //TODO user in here on in app
//#[derive(Clone, Properties)]
//pub struct Props {
    //user: Option<User>,
//}

impl Component for App {
    type Message = Msg;
    //type Properties = Props;
    type Properties = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            current_user: None,
            //props: props,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        //self.props = props;
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => {},
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="App">
                <Nav />
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute | {
                        match switch {
                            AppRoute::Home => html!{ <Home /> },
                            AppRoute::Contact => html!{ <Contact /> },
                            AppRoute::About => html!{ <About /> },
                            AppRoute::Dash => html!{ <Dash /> },
                            AppRoute::Login => html!{ <Login /> },
                            AppRoute::Signup => html!{ <Signup /> },
                            AppRoute::Profile => html!{ <Profile /> },
                            AppRoute::Records => html!{ <Records /> },
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
}
