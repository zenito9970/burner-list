use crate::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddCardProps {
    pub rank: TaskRank,
    pub onedit: Callback<TaskEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddCardState {
    None,
    Editting,
}

#[function_component(AddCard)]
pub fn add_card(props: &AddCardProps) -> Html {
    let state = use_state(|| AddCardState::None);
    let node_ref = use_node_ref();

    let on_edit_start = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.set(AddCardState::Editting);
        })
    };

    let on_edit_end = {
        let rank = props.rank;
        let onedit = props.onedit.clone();
        let state = state.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |_: FocusEvent| {
            if let Some(input) = node_ref.cast::<HtmlTextAreaElement>() {
                if !input.value().is_empty() {
                    onedit.emit(TaskEvent::Add(TaskAddData {
                        rank,
                        value: input.value(),
                    }));
                }
            }
            state.set(AddCardState::None);
        })
    };

    {
        let node_ref = node_ref.clone();
        use_effect(move || {
            if let Some(input) = node_ref.cast::<HtmlTextAreaElement>() {
                input.focus();
            }
            || {}
        });
    }

    match *state {
        AddCardState::Editting => {
            html! {
                <textarea type="text" class="card"
                    ref={node_ref.clone()}
                    onblur={on_edit_end}
                />
            }
        }
        AddCardState::None => {
            html! {
                <div class="card addcard"
                    onclick={on_edit_start}
                >
                    <div class="plus-icon"/>
                    <div>{"追加"}</div>
                </div>
            }
        }
    }
}
