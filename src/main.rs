mod render;
mod state;
mod ui;

fn main() {
    yew::Renderer::<ui::App>::new().render();
}
