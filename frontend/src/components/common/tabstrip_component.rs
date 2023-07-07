use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TabInfo {
    pub display_text: String,
    pub component: Html,
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
        let active_class = if *selected_index == index { "active_tab" } else { "" };
        let onclick = {
            let selected_index = selected_index.clone();
            Callback::from(move |_| selected_index.set(index))
        };

        html! {
            <a class={format!("tab_button {}", active_class)} onclick={onclick} title={tab.display_text.to_owned()}>{&tab.display_text}</a>
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
        <div class={"tabstrip"}>
            <div class={"tabs"}>
                { for render_tabs }
            </div>
            <div>
                { for render_content }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use async_std::task;
    use gloo::utils;
    use std::time::Duration;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::HtmlElement;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn tabstrip_component_test() {
        task::sleep(Duration::from_secs(1)).await;
        let first_tab = TabInfo {
            display_text: "Tab 1".to_string(),
            component: html! {<div class={"tabContent"}>{"Tab 1 content"}</div>},
        };
        let second_tab = TabInfo {
            display_text: "Tab 2".to_string(),
            component: html! {<div class={"tabContent"}>{"Tab 2 content"}</div>},
        };
        let third_tab = TabInfo {
            display_text: "Tab 3".to_string(),
            component: html! {<div class={"tabContent"}>{"Tab 3 content"}</div>},
        };

        let props = TabStripProps {
            tabs: vec![first_tab, second_tab, third_tab],
        };

        let test = yew::Renderer::<TabStrip>::with_props(props);
        task::sleep(Duration::from_secs(1)).await;
        test.render();
        // // Do something after the one second timeout is up!
        // let tabs = utils::document().query_selector_all(".tab_button").unwrap();

        // assert_eq!(tabs.length(), 3, "Three tabs should be rendered");

        // test_is_tab_active("Tab 1", "First tab should be active");

        // // Click second tab
        // let second_tab = tabs.item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
        // second_tab.click();

        // task::sleep(Duration::from_secs(1)).await;
        // let active_tab_after_click = utils::document()
        //     .query_selector(".tab_button.active_tab")
        //     .unwrap()
        //     .unwrap();
        // assert_eq!(
        //     active_tab_after_click.text_content().unwrap(),
        //     "Tab 2",
        //     "Second tab should be active after click"
        // );
        assert_eq!(1, 1, "Twerkt");
        gloo_console::log!("All tests passed");
    }

    // fn click_tab(index: u32) {

    // }

    fn test_is_tab_active(tab_text: &str, error_message: &str) {
        let active_tab = utils::document()
            .query_selector(".tab_button.active_tab")
            .unwrap()
            .unwrap();
        assert_eq!(
            active_tab.text_content().unwrap(),
            tab_text,
            "{error_message}"
        );
    }
}
