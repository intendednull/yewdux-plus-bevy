use async_std::channel::{unbounded, Receiver, Sender};
use std::{ops::Deref, rc::Rc};

use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    pub count: usize,
}

struct StateListener(Sender<State>);
impl Listener for StateListener {
    type Store = State;

    fn on_change(&mut self, state: Rc<Self::Store>) {
        self.0.try_send(state.deref().clone()).ok();
    }
}

pub fn init_channel() -> Receiver<State> {
    let (sender, reciever) = unbounded();
    init_listener(StateListener(sender));
    reciever
}
