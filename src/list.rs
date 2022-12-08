use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, Node};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub rank: TaskRank,
    pub onedit: Callback<TaskEvent>,
}

pub struct TaskList {
    node_ref: NodeRef,
    dispatch_drag_rank: Dispatch<DraggingPositionRank>,
    dispatch_drag_index: Dispatch<DraggingPositionIndex>,
    dispatch_drag_card_id: Dispatch<DraggingCardId>,
}

pub enum TaskListMsg {
    MouseUp,
    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),

    ChangeDragRank,
    ChangeDragIndex,
    ChangeDragCardId,
}

impl Component for TaskList {
    type Message = TaskListMsg;
    type Properties = TaskListProps;

    fn create(ctx: &Context<Self>) -> Self {
        let on_change_drag_rank = ctx.link().callback(move |_| TaskListMsg::ChangeDragRank);
        let on_change_drag_index = ctx.link().callback(move |_| TaskListMsg::ChangeDragIndex);
        let on_change_drag_card_id = ctx.link().callback(move |_| TaskListMsg::ChangeDragCardId);
        Self {
            node_ref: NodeRef::default(),
            dispatch_drag_rank: Dispatch::subscribe(on_change_drag_rank),
            dispatch_drag_index: Dispatch::subscribe(on_change_drag_index),
            dispatch_drag_card_id: Dispatch::subscribe(on_change_drag_card_id),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        let drag_onto_this = self.drag_onto_this(ctx);

        let opt_id = self.dispatch_drag_card_id.get().id;
        let opt_rank = self.dispatch_drag_rank.get().rank;
        let opt_index = self.dispatch_drag_index.get().index;

        use TaskListMsg::*;
        match msg {
            MouseUp => {
                log::debug!("[list] on_mouse_up, rank: {:?}", props.rank);
                if let Some(id) = opt_id {
                    log::debug!("[list] edit, task-id: {}", id);

                    self.dispatch_drag_card_id
                        .reduce(|_| DraggingCardId { id: None });
                    self.dispatch_drag_index
                        .reduce(|_| DraggingPositionIndex { index: None });
                    props.onedit.emit(TaskEvent::Move(TaskMoveData {
                        id,
                        rank: props.rank,
                        index: opt_index,
                    }));

                    return true;
                }
                return false;
            }
            MouseEnter(e) => {
                if let Some(target) = e.target() {
                    if !eq_target_node(target, &self.node_ref) {
                        return false;
                    }
                }
                if opt_rank == Some(props.rank) {
                    return false;
                }

                // log::debug!("[list] on_mouse_enter, rank: {:?}, drag_rank: {:?} -> {:?}", props.rank, opt_rank, Some(props.rank));

                let rank = props.rank;
                self.dispatch_drag_rank
                    .reduce(|_| DraggingPositionRank { rank: Some(rank) });
                return drag_onto_this;
            }
            MouseLeave(e) => {
                if let Some(target) = e.target() {
                    if !eq_target_node(target, &self.node_ref) {
                        return false;
                    }
                }
                if opt_rank == None {
                    return false;
                }

                // log::debug!("[list] on_mouse_leave, rank: {:?}, drag_rank: {:?} -> {:?}", props.rank, opt_rank, None as Option<TaskRank>);

                self.dispatch_drag_rank
                    .reduce(|_| DraggingPositionRank { rank: None });
                self.dispatch_drag_index
                    .reduce(|_| DraggingPositionIndex { index: None });
                return drag_onto_this;
            }

            ChangeDragRank => {
                return drag_onto_this;
            }
            ChangeDragIndex => {
                return drag_onto_this;
            }
            ChangeDragCardId => {
                return opt_rank == Some(props.rank);
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let opt_id = self.dispatch_drag_card_id.get().id;
        let opt_index = self.dispatch_drag_index.get().index;

        let (db, _) = ctx
            .link()
            .context::<Rc<DataBase>>(Callback::noop())
            .expect("context to be set");
        let mut tasks = db.get_by_rank(props.rank);
        if let Some(id) = opt_id {
            tasks.retain(move |task| task.id() != id);
        }

        let drag_onto_this = self.drag_onto_this(ctx);
        let (before_blank, after_blank): (Vec<(&TaskData, usize)>, Vec<(&TaskData, usize)>) =
            if let Some(index) = opt_index {
                let index = tasks.len().min(index);
                let (before, after) = tasks.split_at(index);
                let offset = index; // + if drag_onto_this { 1 } else { 0 };
                let before = before
                    .iter()
                    .enumerate()
                    .map(|(i, &task)| (task, i))
                    .collect();
                let after = after
                    .iter()
                    .enumerate()
                    .map(|(i, &task)| (task, offset + i))
                    .collect();
                (before, after)
            } else {
                let tasks = tasks
                    .iter()
                    .enumerate()
                    .map(move |(i, &task)| (task, i))
                    .collect();
                (tasks, vec![])
            };

        let on_mouse_up = ctx.link().callback(move |_| TaskListMsg::MouseUp);
        let on_mouse_enter = ctx.link().callback(move |e| TaskListMsg::MouseEnter(e));
        let on_mouse_leave = ctx.link().callback(move |e| TaskListMsg::MouseLeave(e));

        html! {
            <div class="task-list"
                ref={self.node_ref.clone()}
                onmouseup={on_mouse_up}
                onmouseenter={on_mouse_enter}
                onmouseleave={on_mouse_leave}
            >
            <div class="task-scroller">
            <div class="task-scroller-inner">

                { for before_blank.iter().map(|(task, i)| html!(
                    <Card id={task.id()} index={*i}
                        onedit={props.onedit.clone()}
                    />
                )) }

                if drag_onto_this {
                    <BlankCard/>
                }

                { for after_blank.iter().map(|(task, i)| html!(
                    <Card id={task.id()} index={*i}
                        onedit={props.onedit.clone()}
                    />
                )) }

                <AddCard rank={props.rank} onedit={props.onedit.clone()}/>

            </div>
            </div>
            </div>
        }
    }
}

impl TaskList {
    fn drag_onto_this(&self, ctx: &Context<Self>) -> bool {
        let opt_id = self.dispatch_drag_card_id.get().id;
        let opt_rank = self.dispatch_drag_rank.get().rank;
        opt_id.is_some() && opt_rank.is_some() && opt_rank == Some(ctx.props().rank)
    }
}

fn eq_target_node(target: EventTarget, node_ref: &NodeRef) -> bool {
    if let Ok(target) = target.dyn_into::<Node>() {
        if let Some(node) = node_ref.get() {
            if target == node {
                return true;
            }
        }
    }

    false
}
