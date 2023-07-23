use std::str::FromStr;

use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub add_relay_cb: Callback<Url>,
    pub relays_set_cb: Callback<MouseEvent>,
}

pub enum Msg {
    // PubkeyChange,
    RelaySubmitted,
}

#[derive(Default)]
pub struct SetRelays {
    relay_node_ref: NodeRef,
}

impl Component for SetRelays {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RelaySubmitted => {
                if let Some(relay_input) = self.relay_node_ref.cast::<HtmlInputElement>() {
                    let relay_input = relay_input.value();

                    let relay = Url::from_str(&relay_input).unwrap();

                    ctx.props().add_relay_cb.emit(relay);
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_relay = ctx.link().callback(|_| Msg::RelaySubmitted);
        html! {
            <>
          <div class="flex justify-center">
            <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-96 max-w-lg">
              <div class="relative z-0 w-full mb-6 group">
                <label for="mint_url" class="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">{"Nostr Relay Url"}</label>
                <input type="text" name="mint_url" id="mint_url" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" ref={self.relay_node_ref.clone()} />
                <div class="flex justify-center">
                <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={set_relay}>{"Add Relay"}</button>
                <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().relays_set_cb.clone()}>{"Next"}</button>
                </div>
              </div>
            </a>
          </div>
        </>
        }
    }
}
