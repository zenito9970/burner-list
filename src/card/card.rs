use crate::prelude::*;
use crate::card::util::*;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub id: Uuid,
    pub index: usize,
    pub onedit: Callback<TaskEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CardState {
    None,
    Editting,
}

pub struct Card {
    state: CardState,
    node_ref: NodeRef,
    dispatch_drag_initial_pos: Dispatch<DraggingInitialPos>,
    dispatch_drag_card_id: Dispatch<DraggingCardId>,
    dispatch_drag_index: Dispatch<DraggingPositionIndex>,
}

pub enum CardMsg {
    EditStart,
    EditEnd,

    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),
}

impl Component for Card {
    type Message = CardMsg;
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: CardState::None,
            node_ref: NodeRef::default(),
            dispatch_drag_initial_pos: Dispatch::subscribe(Callback::noop()),
            dispatch_drag_card_id: Dispatch::subscribe(Callback::noop()),
            dispatch_drag_index: Dispatch::subscribe(Callback::noop()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let id = ctx.props().id;

        use CardMsg::*;
        match msg {
            EditStart => {
                if self.dispatch_drag_card_id.get().id.is_none() {
                    self.state = CardState::Editting;
                    true
                } else {
                    false
                }
            }

            EditEnd => {
                let (db, _) = ctx
                    .link()
                    .context::<Rc<DataBase>>(Callback::noop())
                    .expect("context to be set");
                let value = db
                    .get_by_id(id)
                    .map(move |task| task.value.clone())
                    .expect("!!!error!!!");

                if let Some(input) = self.node_ref.cast::<HtmlTextAreaElement>() {
                    let input_value = input.value();
                    log::debug!("blur, id: {}, value: {}", id, input_value);
                    if input_value.is_empty() {
                        ctx.props().onedit.emit(TaskEvent::Delete(TaskDeleteData { id }));
                    } else if value != input_value {
                        ctx.props().onedit.emit(TaskEvent::Edit(TaskEditData {
                            id,
                            value: input_value,
                        }));
                    }
                }

                self.state = CardState::None;
                true
            }

            MouseDown(e) => {
                log::debug!("[card] mouse_down");
                let pos = Some((e.offset_x(), e.offset_y()));
                self.dispatch_drag_initial_pos
                    .reduce(|_| DraggingInitialPos { pos });
                false
            }
            MouseMove(e) => {
                let id = ctx.props().id;
                let initial_pos = self.dispatch_drag_initial_pos.get().pos;
                let drag_id = self.dispatch_drag_card_id.get().id;
                let drag_index = self.dispatch_drag_index.get().index;
                // log::debug!("[card] mouse_move");

                if initial_pos.is_some() && drag_id.is_none() {
                    log::debug!("[card] start dragging");
                    self.dispatch_drag_card_id
                        .reduce(|_| DraggingCardId { id: Some(id) });
                    let index = ctx.props().index;
                    self.dispatch_drag_index
                        .reduce(|_| DraggingPositionIndex { index: Some(index) });
                    return false;
                }

                let mut index = ctx.props().index;
                if e.movement_y() > 0 && drag_index.is_some() {
                    index += 1;
                }
                if drag_index != Some(index) {
                    self.dispatch_drag_index
                        .reduce(|_| DraggingPositionIndex { index: Some(index) });
                }
                false
            }
            MouseEnter(_) => {
                // let index = ctx.props().index;
                // self.dispatch_drag_index
                //     .reduce(|_| DraggingPositionIndex { index: Some(index) });
                false
            }
            MouseLeave(_) => {
                // self.dispatch_drag_index
                //     .reduce(|_| DraggingPositionIndex { index: None });
                false
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(input) = self.node_ref.cast::<HtmlTextAreaElement>() {
            input.focus();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (db, _) = ctx
            .link()
            .context::<Rc<DataBase>>(Callback::noop())
            .expect("context to be set");
        let value = db
            .get_by_id(ctx.props().id)
            .map(move |task| task.value.clone())
            .expect("!!!error!!!");

        let on_mouse_up = ctx.link().callback(move |_| CardMsg::EditStart);
        let on_edit_end = ctx.link().callback(move |_| CardMsg::EditEnd);
        let on_mouse_down = ctx.link().callback(move |e| CardMsg::MouseDown(e));
        let on_mouse_move = ctx.link().callback(move |e| CardMsg::MouseMove(e));
        let on_mouse_enter = ctx.link().callback(move |e| CardMsg::MouseEnter(e));
        let on_mouse_leave = ctx.link().callback(move |e| CardMsg::MouseLeave(e));

        match self.state {
            CardState::Editting => {
                html! {
                    <textarea type="text" class="card"
                        ref={self.node_ref.clone()}
                        value={value}
                        onblur={on_edit_end}
                    />
                }
            }
            _ => {
                html! {
                    <p class="card"
                        ref={self.node_ref.clone()}
                        onmousedown={on_mouse_down}
                        onmousemove={on_mouse_move}
                        onmouseup={on_mouse_up}
                        onmouseenter={on_mouse_enter}
                        onmouseleave={on_mouse_leave}
                    >
                    // { format!("id: {}, value: ", props.id) }
                    { task_value_as_html(&value) }
                    </p>
                }
            }
        }
    }
}
