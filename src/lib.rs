extern crate sha1;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Hasher {
    link: ComponentLink<Self>,
    sha1: String,
    title: String,
    value: String,
}

enum Msg {
    GotInput(String)
}

impl Component for Hasher {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Hasher {
            link,
            sha1: String::from(""),
            title: String::from("hasher"),
            value: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut m = sha1::Sha1::new();
        match msg {
            Msg::GotInput(new_value) => {
                m.update(new_value.as_bytes());
                self.value = m.digest().to_string();
            },
            _ => {},
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container" style="margin-top: 100px;">
                <div class="row">
                    <header class="title tc">
                        <h1>{&self.title}</h1>
                    </header>
                </div>
                <div class="row tc user">
                    <input class="user tc" type="text" oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value)) />
                    <p>{&self.value}</p>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Hasher>::new().mount_to_body();
}