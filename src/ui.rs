use yew::prelude::*;
use yewdux::prelude::*;

use crate::{render::Render, state::State};

#[function_component]
pub fn App() -> Html {
    let (state, dispatch) = use_store::<State>();
    let button = {
        let onclick = dispatch.reduce_mut_callback(|s| s.count += 1);
        html! {
            <button style="width: 100px; height: 100px" {onclick}>{"+1"}</button>
        }
    };

    html! {
        <>
        <p>{state.count}</p>
        <div>{button}</div>
        <Render />
        </>
    }
}
