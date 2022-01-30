use {
    crate::header::Header,
    yew::{classes, prelude::*},
};

pub enum CounterMsg {
    // SetReciever(Scope<App>),
    AddOne,
    SubOne,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Counter {
    value: u64,
}

impl Component for Counter {
    type Message = CounterMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CounterMsg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            CounterMsg::SubOne => {
                self.value = self.value.saturating_sub(1);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let inc = link.callback(|_| CounterMsg::AddOne);
        let dec = link.callback(|_| CounterMsg::SubOne);
        html! {
          <div style="display: inline-block;width: auto;margin: 4px;padding:8px;background-color: pink; border: solid black 3px; border-radius: 8px;">
            <div class={classes!((0 < self.value).then(|| Some("heading")))}>
              { "A counter" }
            </div>
            <div class={classes!({"buttons"})}>
              <button class={classes!("button")} onclick={inc} >
                { "+ 1" }
              </button>
              <button class={classes!("button")} onclick={dec}>
                 { "- 1" }
              </button>
            </div>
            <p>{ format!("The value is {}.", self.value) }</p>
          </div>
        }
    }
}
