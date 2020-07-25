use yew::{html, Component, ComponentLink, FocusEvent, Callback, agent, Html, Properties, ShouldRender};
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::models::user::User;
use crate::errors::AuthError;

/// Login page
pub struct Login {
    req_user: User,
    req_pass: String,
    //props: Props,
    link: ComponentLink<Self>,
}

//#[derive(Clone, Properties)]
//pub struct Props {
    //callback: Callback<User>,
//}
pub enum Msg {
    RequestSubmitted,
    ResponseReceived(Result<User, AuthError>),
    ForgotUsername,
    ForgotPassword,
    RequestError,
    ResponseError,

}

impl Component for Login {
    type Message = Msg;
    //type Properties = Props;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            req_user: User {
                username: "".to_string(),
                uid: None
            },
            req_pass: "".to_string(),
            //props, link
            link
        }
    }

    //fn change(&mut self, props: Self::Properties) -> ShouldRender {
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => { println!("requesting login..."); }
            _ => {},
        }
        true
    }

    fn view(&self) -> Html {
        let handle_submit = self.link.callback(|event: FocusEvent| {
            Msg::Request
        });

        html! {
            <div class="auth-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "Sign In" }</h1>
                            <p class="text-xs-center">
                                <RouterAnchor<AppRoute> route=AppRoute::Signup>
                                    { "Sign Up" }
                                </RouterAnchor<AppRoute>>
                            </p>
                            <form onsubmit=handle_submit>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="username"
                                            value=&self.req_user.username
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control"
                                            type="password"
                                            value=&self.req_pass
                                            />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg"
                                        type="submit"
                                        disabled=false>
                                        { "Login" }
                                    </button>
                                </fieldset>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
