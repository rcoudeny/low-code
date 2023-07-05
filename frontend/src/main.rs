mod api;
mod components;
mod store;

// use store::Store;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::{
    common::tabstrip::{TabInfo, TabStrip},
    pagetest::PageTest,
};

#[function_component]
fn App() -> Html {
    // let (store, _) = use_store::<Store>();

    html! {
             <div>
        <TabStrip tabs={vec![ TabInfo { display_text: "Data type" .to_string(), component: html! {<PageTest
            test="Wa is dees?" />}
        },
        TabInfo {
        display_text: "Formulier".to_string(),
        component: html! {
        <PageTest test="Jajaja wi" />}
        },
        TabInfo {
        display_text: "Result".to_string(),
        component: html! {
        <PageTest test="Wajow tis waar" />}
        },
        ]}/>
    </div>
         }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
