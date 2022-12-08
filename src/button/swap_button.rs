use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SwapButtonProps {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(SwapButton)]
pub fn swap_button(props: &SwapButtonProps) -> Html {
    html! {
        <button class="swap-button" onclick={props.onclick.clone()}>
        <div class="swap-icon"></div>
        </button>
    }
}
