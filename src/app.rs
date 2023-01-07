use crate::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

pub struct App {
    db: DataBase,
}

pub enum AppMsg {
    TaskEvent(TaskEvent),
    Burn,
    Swap,
}

impl<'a> Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            db: DataBase::load_from_local_storage(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AppMsg::*;
        let res = match msg {
            TaskEvent(e) => self.db.apply_event(&e),
            Burn => {
                self.db.burn_tasks(TaskRank::Primary);
                true
            }
            Swap => {
                self.db.swap_tasks(TaskRank::Primary, TaskRank::Secondary);
                true
            }
        };

        self.db.save_to_local_storage();
        res
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_edit = ctx.link().callback(move |e| AppMsg::TaskEvent(e));
        let on_burn = ctx.link().callback(move |_| AppMsg::Burn);
        let on_swap = ctx.link().callback(move |_| AppMsg::Swap);
        html! {
            <ContextProvider<Rc<DataBase>> context={Rc::new(self.db.clone())}>
                <main>
                    // <StoreWatcher/>
                    <DraggableArea/>
                    <div class="split">
                        <div class="split-item" style="margin-right: -10px">
                            <TaskList rank={TaskRank::Primary} onedit={on_edit.clone()}/>
                        </div>

                        <div class="split-item">
                            <div class="split-right">
                                <div class="split-right-item" style="margin-bottom: -10px">
                                    <TaskList rank={TaskRank::Secondary} onedit={on_edit.clone()}/>
                                </div>

                                <div class="split-right-item">
                                    <TaskList rank={TaskRank::Other} onedit={on_edit.clone()}/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <SwapButton onclick={on_swap}/>
                    <BurnButton onclick={on_burn}/>
                    // <DeleteArea onedit={on_edit.clone()}/>
                    // <DraggingCard/>
                </main>
            </ContextProvider<Rc<DataBase>>>
        }
    }
}
