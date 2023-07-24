use cashu_crab::Amount;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub amount_cb: Callback<Amount>,
}

pub enum Msg {
    // AmountChange,
    AmountSubmitted,
    ButtonPressed(i32),
}

#[derive(Default)]
pub struct Pos {
    amount: String,
}

impl Component for Pos {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            amount: "0".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AmountSubmitted => {
                if let Ok(amount) = self.amount.parse() {
                    let amount = Amount::from_sat(amount);

                    ctx.props().amount_cb.emit(amount);
                }

                true
            }
            Msg::ButtonPressed(button_num) => {
                log::debug!("{}", button_num);

                if button_num.eq(&12) {
                    self.amount.pop();
                    if self.amount.eq("") {
                        self.amount = format!("{}", 0);
                    }
                } else {
                    let button_num = if button_num.eq(&11) { 0 } else { button_num };

                    if self.amount.eq("0") {
                        self.amount = format!("{}", button_num);
                    } else {
                        self.amount = format!("{}{}", self.amount, button_num);
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_pubkey = ctx.link().callback(|_| Msg::AmountSubmitted);
        let l = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        html! {
          <div class="flex justify-center">
            <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-96 lg:max-w-lg sm:w-full">

            <h1 class="text-3xl mb-4 font-semibold leadi text-center">{format!("{} sats", self.amount.clone())}</h1>
            <div class="grid grid-cols-3 gap-4">

            {
                l.into_iter().map(|i| {
                    let cb = ctx.link().callback(move |_| Msg::ButtonPressed(i.clone()));
                    if i == 10 {
                        html!{
                            <>
                            <div></div>
                            </>
                        }
                    } else if i == 11 {
                        html!{
                            <>
                            <button class="px-8 py-4 rounded-sm shadow-lg dark:bg-violet-400 dark:text-gray-900 lg:text-lg sm:text-5xl font-medium" onclick={cb}>{"0"}</button>
                            </>
                        }
                    } else if i == 12 {
                        html! {
                            <>
                            <button class="px-8 py-4 rounded-sm shadow-lg dark:bg-violet-400 dark:text-gray-900 lg:text-lg sm:text-5xl font-medium" onclick={cb}>{"X"}</button>
                            </>
                        }
                    } else {
                        html! {
                            <>
                            <button class="px-8 py-4 rounded-sm shadow-lg dark:bg-violet-400 dark:text-gray-900 lg:text-lg sm:text-5xl font-medium" onclick={cb}>{i}</button>
                            </>
                        }
                    }
                }).collect::<Html>()
            }
            </div>

              <div class="relative z-0 w-full mb-6 group">
                    // <input type="numeric" name="amount" id="amount" class="block py-4 px-6 w-full lg:text-lg sm:text-5xl text-gray-900 bg-transparent border-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:border-blue-600 peer" placeholder={"Amount (sats)"} ref={self.amount_node_ref.clone()} />
                <div class="flex justify-center">
                    <button class="px-8 py-4 mt-5 w-full rounded-sm shadow-lg dark:bg-green-600 dark:text-gray-900 lg:text-lg sm:text-5xl font-medium" onclick={set_pubkey}>{"Create Invoice"}</button>
                </div>
              </div>
            </a>
          </div>
        }
    }
}
