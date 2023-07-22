use std::str::FromStr;

use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub mint_set_cb: Callback<Url>,
}

pub enum Msg {
    // PubkeyChange,
    PubkeySubmitted,
}

#[derive(Default)]
pub struct SetMint {
    mint_node_ref: NodeRef,
}

impl Component for SetMint {
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
                if let Some(mint_input) = self.mint_node_ref.cast::<HtmlInputElement>() {
                    let mint_input = mint_input.value();

                    let mint = Url::from_str(&mint_input).unwrap();

                    ctx.props().mint_set_cb.emit(mint);
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_pubkey = ctx.link().callback(|_| Msg::PubkeySubmitted);
        html! {
            <>
            <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
            <div class="relative z-0 w-full mb-6 group">
                <label for="description" class="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">{"Description"}</label>
                <input type="text" name="description" id="description" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" ref={self.mint_node_ref.clone()} />
                <button class="px-6 py-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={set_pubkey}>{"Set Mint"}</button>
            </div>
            </a>
            </>
        }
    }
}
