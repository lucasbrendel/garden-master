#[macro_use] extern crate yew;
use yew::prelude::*;
extern crate chrono;
extern crate clap;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde_rusqlite;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate log;
extern crate simplelog;

mod logging;

#[allow(dead_code)]
mod db;

use clap::App;
use db::DataMgr;
use logging::logging_init;

struct Model { }

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
}


fn main() {
    logging_init();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}



