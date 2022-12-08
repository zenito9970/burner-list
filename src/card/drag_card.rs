use crate::prelude::*;
use crate::card::util::*;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(DraggingCard)]
pub fn dragging_card() -> Html {
    let db = use_context::<Rc<DataBase>>().expect("context to be set");
    let card_id = use_store::<DraggingCardId>().0.id;
    let value = card_id
        .map(|id| db.get_by_id(id).map(|task| task.value.clone()))
        .unwrap_or_default()
        .unwrap_or_default();

    let mut class = classes!("card", "dragging-card");
    if card_id.is_none() {
        class.push("invisible");
    }

    let (initial_pos, _) = use_store::<DraggingInitialPos>();
    let (mouse_pos, _) = use_store::<DraggingMousePos>();
    let (x, y) = get_pos(&mouse_pos, &initial_pos).unwrap_or((0, 0));
    let style = format!("left: {}px; top: {}px", x, y);
    // let style = format!("transform: translate({}px, {}px)", x, y);

    html! {
        <p class={class} style={style}>
        { task_value_as_html(&value) }
        </p>
    }
}

fn get_pos(
    mouse_pos: &Rc<DraggingMousePos>,
    initial_pos: &Rc<DraggingInitialPos>,
) -> Option<(i32, i32)> {
    let init_pos = initial_pos.pos?;
    let mouse_pos = mouse_pos.pos?;
    Some(transform(init_pos, mouse_pos))
}

fn transform((init_x, init_y): (i32, i32), (orig_x, orig_y): (i32, i32)) -> (i32, i32) {
    let rad_3 = 3.0_f64.to_radians();
    let (sin, cos, tan) = (rad_3.sin(), rad_3.cos(), rad_3.tan());
    let (init_x, init_y) = (init_x as f64, init_y as f64);
    let (orig_x, orig_y) = (orig_x as f64, orig_y as f64);

    let offset_x = (init_x - init_y * tan) * cos;
    let offset_y = (init_x - init_y * tan) * sin + init_y / cos;

    let x = orig_x - offset_x;
    let y = orig_y - offset_y;
    (x as i32, y as i32)
}
