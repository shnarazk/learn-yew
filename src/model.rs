use {
    crate::header::Header,
    yew::{classes, prelude::*},
};

pub enum Msg {
    AddOne,
    SubOne,
}

pub struct Model {
    value: u64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubOne => {
                self.value = self.value.saturating_sub(1);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <Header/>
                <div class={classes!((0 < self.value).then(|| Some("heading")))}>{ "A span" }</div>
                <div class={classes!({"buttons"})}>
                    <button class={classes!({"button"})} onclick={link.callback(|_| Msg::AddOne)}>{ "+ 1" }</button>
                    <button class={classes!({"button"})} onclick={link.callback(|_| Msg::SubOne)}>{ "- 1" }</button>
                </div>
                <p>{ format!("The value is {}.", self.value) }</p>
            </div>
        }
    }
}
