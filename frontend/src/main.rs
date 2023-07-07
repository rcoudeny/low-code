mod api;
mod components;
mod store;

// use store::Store;
use yew::prelude::*;

use crate::components::{
    common::tabstrip_component::{TabInfo, TabStrip},
    datatype::datatype_component::DataType,
    form::{form_component::Form, form_editor_component::FormEditor},
};

#[function_component]
fn HyphenApp() -> Html {
    // let (store, _) = use_store::<Store>();

    html! {
             <div>
        <TabStrip tabs={vec![ TabInfo {
            display_text: "Data type" .to_string(),
            component: html! {<DataType />}
        },
        TabInfo {
        display_text: "Formulier".to_string(),
        component: html! {
        <FormEditor/>}
        },
        TabInfo {
        display_text: "Result".to_string(),
        component: html! {
        <Form />}
        },
        ]}/>
    </div>
         }
}

fn main() {
    yew::Renderer::<HyphenApp>::new().render();
}
