use yew::prelude::*;

/// Signup page
pub struct Signup;

impl Component for Signup {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Signup {}
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
                    <p>
                        <a
                            class="app-link"
                            href="https://github.com/jetli/create-yew-app"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "test Yew App" }
                        </a>
                        { ", Set up a modern yew web app by running one command." }
                    </p>
                    <p>
                        { "Edit " } <code>{ "src/components/Signup.rs" }</code> { " and save to reload." }
                    </p>
                </header>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::Signup;
    use yew::App;

    #[wasm_bindgen_test]
    fn signup_page_has_an_app_link() {
        let app: App<Signup> = yew::App::new();
        app.mount(yew::utils::document().get_element_by_id("output").unwrap());

        let app_links = yew::utils::document().get_elements_by_class_name("app-link");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
