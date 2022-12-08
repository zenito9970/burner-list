use crate::prelude::*;
use serde_json::json;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct StoreWatcher {
    card_id: Dispatch<DraggingCardId>,
    initial_pos: Dispatch<DraggingInitialPos>,
    mouse_pos: Dispatch<DraggingMousePos>,
    index: Dispatch<DraggingPositionIndex>,
    rank: Dispatch<DraggingPositionRank>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StoreWatcherMsg {
    ChangeValue,
    SerializeButton,
}

impl Component for StoreWatcher {
    type Message = StoreWatcherMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            card_id: Dispatch::subscribe(
                ctx.link().callback(move |_| StoreWatcherMsg::ChangeValue),
            ),
            initial_pos: Dispatch::subscribe(
                ctx.link().callback(move |_| StoreWatcherMsg::ChangeValue),
            ),
            mouse_pos: Dispatch::subscribe(
                ctx.link().callback(move |_| StoreWatcherMsg::ChangeValue),
            ),
            index: Dispatch::subscribe(ctx.link().callback(move |_| StoreWatcherMsg::ChangeValue)),
            rank: Dispatch::subscribe(ctx.link().callback(move |_| StoreWatcherMsg::ChangeValue)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if msg == StoreWatcherMsg::SerializeButton {
            let (db, _) = ctx
                .link()
                .context::<Rc<DataBase>>(Callback::noop())
                .expect("context to be set");
            log::debug!("[debug] db: {}", json!(db));
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let card_id = self.card_id.get().id;
        let initial_pos = self.initial_pos.get().pos;
        let mouse_pos = self.mouse_pos.get().pos;
        let index = self.index.get().index;
        let rank = self.rank.get().rank;

        let on_click_serialize = ctx
            .link()
            .callback(move |_| StoreWatcherMsg::SerializeButton);

        let style = r#"
            position: fixed;
            pointer-events: none;
            background-color: rgba(0,0,0,0.5);
            color: #fff;
            bottom: 10px;
            left: 50%;
        "#
        .replace(" ", "")
        .replace("\n", "");

        html! {
            <div style={style}>
                <div> { format!("card_id: {:?}", card_id) } </div>
                <div> { format!("initial_pos: {:?}", initial_pos) } </div>
                <div> { format!("mouse_pos: {:?}", mouse_pos) } </div>
                <div> { format!("index: {:?}", index) } </div>
                <div> { format!("rank: {:?}", rank) } </div>
                <button onclick={on_click_serialize}> { "serialize" } </button>
            </div>
        }
    }
}
