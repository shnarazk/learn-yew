use {
    crate::{counter::Counter, header::Header},
    yew::{classes, prelude::*, Properties, html},
};

#[derive(Default, Properties, PartialEq)]
pub struct ModelProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Counter>,
}

pub enum Msg {
    AddOne,
    SubOne,
}

pub struct Model {
    value: u64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ModelProps;

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

    // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let s = S::default();
        html! {
            <div>
                <Header/>
                {"aaa"}
                <div class="list">
                  { for ctx.props().children.iter() }
                </div>
                {"end"}
                { s.render() }
              <div style="display: inline-block; width: auto; padding: 8px; border: solid black 3px; border-radius: 8px;">
                <div class={classes!((0 < self.value).then(|| Some("heading")))}>{ "Counter" }</div>
                <div class={classes!({"buttons"})}>
                    <button class={classes!({"button"})} onclick={link.callback(|_| Msg::AddOne)}>{ "+ 1" }</button>
                    <button class={classes!({"button"})} onclick={link.callback(|_| Msg::SubOne)}>{ "- 1" }</button>
                </div>
                <p>{ format!("The value is {}.", self.value) }</p>
              </div>
              <Counter />
            </div>
        }
    }
}

// ///
#[derive(Default)]
struct S {
    value: usize
}

impl S {
    pub fn render(&self) -> Html {
        html! { <div style="color: red;">{ format!("S is Rendered {}", self.value) }</div> }
    }
}


    
