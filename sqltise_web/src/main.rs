use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude;

struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // should rerender
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };
        html! {
            <div>
                <button onclick=&self.onclick>{ button_text }</button>
                <button onfart={ "fart" }>{ "hello" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<App>();
}
