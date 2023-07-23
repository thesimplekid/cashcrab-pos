use cashu_crab::Amount;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub amount_cb: Callback<Amount>,
}

pub enum Msg {
    // AmountChange,
    AmountSubmitted,
}

#[derive(Default)]
pub struct Pos {
    amount_node_ref: NodeRef,
}

impl Component for Pos {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AmountSubmitted => {
                if let Some(amount_input) = self.amount_node_ref.cast::<HtmlInputElement>() {
                    if let Ok(amount) = amount_input.value().parse() {
                        let amount = Amount::from_sat(amount);

                        ctx.props().amount_cb.emit(amount);
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_pubkey = ctx.link().callback(|_| Msg::AmountSubmitted);
        html! {
          <div class="flex justify-center">
            <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-96 lg:max-w-lg sm:w-full">
              <div class="relative z-0 w-full mb-6 group">
                    <input type="numeric" name="amount" id="amount" class="block py-4 px-6 w-full lg:text-lg sm:text-5xl text-gray-900 bg-transparent border-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:border-blue-600 peer" placeholder={"Amount (sats)"} ref={self.amount_node_ref.clone()} />
                <div class="flex justify-center">
                    <button class="px-8 py-4 rounded-sm shadow-lg dark:bg-violet-400 dark:text-gray-900 lg:text-lg sm:text-5xl font-medium" onclick={set_pubkey}>{"Create Invoice"}</button>
                </div>
              </div>
            </a>
          </div>
        }
    }
}
