use crate::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct DraggableArea {
    dispatch_id: Dispatch<DraggingCardId>,
    dispatch_initial_pos: Dispatch<DraggingInitialPos>,
    dispatch_mouse_pos: Dispatch<DraggingMousePos>,

    on_mouse_move: Option<Closure<dyn Fn(MouseEvent)>>,
    on_mouse_up: Option<Closure<dyn Fn(MouseEvent)>>,
}

impl Component for DraggableArea {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dispatch_id: Dispatch::subscribe(Callback::noop()),
            dispatch_initial_pos: Dispatch::subscribe(Callback::noop()),
            dispatch_mouse_pos: Dispatch::subscribe(Callback::noop()),
            on_mouse_move: None,
            on_mouse_up: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let on_mouse_move = {
            self.dispatch_mouse_pos
                .reduce_mut_callback_with(move |s, e: MouseEvent| {
                    // log::debug!("[document] mousemove");
                    s.pos = Some((e.page_x(), e.page_y()));
                })
        };

        let on_mouse_up_state = {
            self.dispatch_initial_pos.reduce_mut_callback(move |s| {
                s.pos = None;
            })
        };

        let on_mouse_up_id = {
            self.dispatch_id.reduce_mut_callback(move |s| {
                s.id = None;
            })
        };

        let on_mouse_move =
            Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |e| on_mouse_move.emit(e)));
        let on_mouse_up = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |e| {
            on_mouse_up_state.emit(e.clone());
            on_mouse_up_id.emit(e);
        }));

        let document = web_sys::window()
            .expect("window")
            .document()
            .expect("document");
        document
            .add_event_listener_with_callback("mousemove", on_mouse_move.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())
            .unwrap();

        self.on_mouse_move = Some(on_mouse_move);
        self.on_mouse_up = Some(on_mouse_up);
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let document = web_sys::window()
            .expect("window")
            .document()
            .expect("document");
        if let Some(on_mouse_move) = self.on_mouse_move.take() {
            document
                .add_event_listener_with_callback(
                    "mousemove",
                    on_mouse_move.as_ref().unchecked_ref(),
                )
                .unwrap();
        }
        if let Some(on_mouse_up) = self.on_mouse_up.take() {
            document
                .add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="draggable-area">
                <DraggingCard/>
            </div>
        }
    }
}
