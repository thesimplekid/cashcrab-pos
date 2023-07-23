use std::str::FromStr;

use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub add_relay_cb: Callback<MouseEvent>,
    pub set_pubkey_cb: Callback<MouseEvent>,
    pub set_mint_cb: Callback<MouseEvent>,
    pub home_cb: Callback<MouseEvent>,
}

#[derive(Default)]
pub struct Settings;

impl Component for Settings {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
          <div class="flex justify-center">
            <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-96 max-w-lg">
              <div class="relative z-0 w-full mb-6 group">
                <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().set_pubkey_cb.clone()}>{"Set Receiver"}</button>
                <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().add_relay_cb.clone()}>{"Add relay"}</button>
                <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().set_mint_cb.clone()}>{"Set Mint"}</button>
                <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().set_mint_cb.clone()}>{"Home"}</button>
                </div>
            </a>
          </div>
        </>
        }
    }
}
