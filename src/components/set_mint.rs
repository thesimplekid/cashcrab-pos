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
    MintSubmitted,
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
            Msg::MintSubmitted => {
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
        let set_mint = ctx.link().callback(|_| Msg::MintSubmitted);
        html! {
            <>
            <div class="flex justify-center">
          <a class="block p-8 bg-white border border-gray-200 rounded-lg shadow-lg hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-full lg:w-1/2">
            <div class="relative z-0 w-full mb-8 group">
              <input type="text" name="mint_url" id="mint_url" class="block py-4 px-6 w-full text-5xl lg:text-lg text-gray-900 bg-transparent border-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:border-blue-600 peer" placeholder={"Mint Url"} ref={self.mint_node_ref.clone() } />
              <div class="flex justify-center mt-8">
                <button class="px-8 py-4 rounded-sm shadow-lg dark:bg-violet-400 dark:text-gray-900 text-5xl lg:text-xl font-medium" onclick={set_mint}>{"Set Mint"}</button>
              </div>
            </div>
          </a>
        </div>
        </>
        }
    }
}
