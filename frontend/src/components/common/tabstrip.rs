use gloo_console::log;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TabInfo {
    pub component: Html,
    pub display_text: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TabStripProps {
    pub tabs: Vec<TabInfo>,
}

#[function_component]
pub fn TabStrip(props: &TabStripProps) -> Html {
    let tabs: &Vec<TabInfo> = &props.tabs;
    let selected_index = use_state(|| 0);

    let render_tabs = tabs.iter().enumerate().map(|(index, tab)| {
        let active_class = if *selected_index == index {"text-decoration:underline;"} else {""};
        let onclick = {
            let selected_index = selected_index.clone();
            Callback::from(move |_| selected_index.set(index))
        };

        html! {
            <a style={format!("cursor: pointer; margin-right: 1em;{}", active_class )} onclick={onclick}>{&tab.display_text}</a>
        }
    });

    let render_content = tabs.iter().enumerate().map(|(index, tab)| {
        let is_selected = *selected_index == index;
        if is_selected {
            html! { {tab.component.clone()} }
        } else {
            html! {}
        }
    });

    html! {
        <div style="border: dotted; margin: 1em; padding: 1em">
            <div style="border-bottom: dotted; margin-bottom: 2em; padding-bottom: 1em">
                { for render_tabs }
            </div>
            <div>
                { for render_content }
            </div>
        </div>
    }
}
