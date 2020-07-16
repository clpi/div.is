use yew::prelude::*;

/// contac page
pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Contact {}
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
                    <h1>{ "Contact" }</h1>
                    <p>
                        { "Contact page" }
                    </p>
                    <p>
                        { "Edit " } <code>{ "src/components/about.rs" }</code> { " and save to reload." }
                    </p>
                </header>
            </div>
        }
    }
}

