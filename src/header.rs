use yew::{classes, prelude::*};

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("header")}>{ "Header" }</div>
        }
    }
}
