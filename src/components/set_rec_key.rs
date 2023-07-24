use nostr_sdk::key::FromPkStr;
use nostr_sdk::Keys;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::bindings;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub logged_in_callback: Callback<Keys>,
}

async fn _get_pubkey() -> Option<String> {
    let key = bindings::get_pubkey().await;
    key.as_string()
}

pub enum Msg {
    // PubkeyChange,
    PubkeySubmitted,
}

#[derive(Default)]
pub struct SetRecKey {
    pubkey_node_ref: NodeRef,
}

impl Component for SetRecKey {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PubkeySubmitted => {
                if let Some(pubkey_input) = self.pubkey_node_ref.cast::<HtmlInputElement>() {
                    let pubkey_input = pubkey_input.value();

                    let pubkey = Keys::from_pk_str(&pubkey_input).unwrap();

                    ctx.props().logged_in_callback.emit(pubkey);
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_pubkey = ctx.link().callback(|_| Msg::PubkeySubmitted);
        html! {
          <div class="flex justify-center">
            <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-96 w-full lg:max-w-lg">
              <div class="relative z-0 w-full mb-6 group">
                <input type="text" name="description" id="description" class="block py-4 px-6 w-full text-5xl  lg:text-lg text-gray-900 bg-transparent border-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:border-blue-600 peer" placeholder={"Nostr Pubkey"} ref={self.pubkey_node_ref.clone()} />
                <div class="flex justify-center">
                <button class="px-8 py-4 rounded-sm shadow-lg dark:bg-violet-400 dark:text-gray-900  text-5xl lg:text-xl font-medium" onclick={set_pubkey}>{"Set Receive Key"}</button>
                </div>
              </div>
            </a>
          </div>
        }
    }
}
