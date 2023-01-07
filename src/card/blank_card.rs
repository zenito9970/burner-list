use crate::card::util::*;
use crate::prelude::*;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(BlankCard)]
pub fn blank_card() -> Html {
    let db = use_context::<Rc<DataBase>>().expect("context to be set");
    let card_id = use_store::<DraggingCardId>().0.id;
    let value = card_id
        .map(|id| db.get_by_id(id).map(|task| task.value.clone()))
        .unwrap_or_default()
        .unwrap_or_default();

    html! {
        <p class="card blank-card">
        <p>
        { task_value_as_html(&value) }
        </p>
        </p>
    }
}
