use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    engine::{self, CANVAS_ID},
    state::{self, State},
};

#[function_component]
pub fn App() -> Html {
    use_effect_with_deps(
        |_| {
            let receiver = state::init_channel();
            engine::start(receiver);
            || {}
        },
        (),
    );

    let button = {
        let onclick = Dispatch::<State>::new().reduce_mut_callback(|s| s.count += 1);
        html! {
            <button style="width: 100px; height: 100px" {onclick}>{"+1"}</button>
        }
    };

    html! {
        <>
        <div>{button}</div>
        <canvas id={CANVAS_ID} />
        </>
    }
}
