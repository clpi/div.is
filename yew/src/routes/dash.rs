use yew::prelude::*;

/// Dash page
pub struct Dash;

impl Component for Dash {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Dash {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <header class="app-header">
                    <h1>{ "Dashboard" }</h1>
                    <p>
                        { ", Set up a modern yew web app by running one command." }
                    </p>
                    <p>
                        { "Edit " } <code>{ "src/components/about.rs" }</code> { " and save to reload." }
                    </p>
                </header>
            </div>
        }
    }
}
