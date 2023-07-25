use anyhow::Result;
use cashu_crab::Amount;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub amount_cb: Callback<(Amount, String)>,
}

pub enum Msg {
    // AmountChange,
    AmountSubmitted,
    ButtonPressed(i32),
    GotPrice(u64),
    EnterFiat,
}

#[derive(Default)]
pub struct Pos {
    amount: String,
    sat_per_usd: Option<u64>,
    fiat_value: String,
    enter_fiat: bool,
    disable_decimal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsdPrice {
    usd: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinGeckoPrice {
    bitcoin: UsdPrice,
}

async fn get_price(price_cb: Callback<u64>) -> Result<()> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let price: CoinGeckoPrice = Request::get(url).send().await?.json().await?;

    log::debug!("{:?}", price);

    let sats_per_dollar = (1.0 / (price.bitcoin.usd as f64) * 100000000.0).round() as u64;

    price_cb.emit(sats_per_dollar);

    Ok(())
}

fn format_fiat(amount: &str) -> String {
    let amount = amount.replace('.', "").trim_start_matches('0').to_string();
    let num_zeros = if amount.len().lt(&3) {
        3 - amount.len()
    } else {
        0
    };

    let formated_amount = format!("{}{}", "0".repeat(num_zeros), amount);
    let decimal_pos = formated_amount.len() - 2;
    let mut result = formated_amount;
    result.insert(decimal_pos, '.');

    result
}

impl Component for Pos {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let price_cb = ctx.link().callback(Msg::GotPrice);

        spawn_local(async move {
            get_price(price_cb).await.ok();
        });

        Self {
            amount: "0".to_string(),
            fiat_value: "0.00".to_string(),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AmountSubmitted => {
                if let Ok(amount) = self.amount.parse() {
                    let amount = Amount::from_sat(amount);

                    ctx.props()
                        .amount_cb
                        .emit((amount, self.fiat_value.clone()));
                }

                true
            }
            Msg::ButtonPressed(button_num) => {
                let edit_amount = match self.enter_fiat {
                    true => &self.fiat_value,
                    false => &self.amount,
                };

                let new_amount = match button_num {
                    12 => {
                        let mut edit_amount = edit_amount.clone();
                        let popped = edit_amount.pop();
                        if self.enter_fiat {
                            format_fiat(&edit_amount)
                        } else if edit_amount.is_empty() {
                            "0".to_string()
                        } else if let Some(ch) = popped {
                            if ch == '.' {
                                self.disable_decimal = false;
                            }
                            edit_amount
                        } else {
                            edit_amount
                        }
                    }
                    10 => {
                        self.disable_decimal = true;
                        format!("{}.", edit_amount)
                    }
                    11 => {
                        if edit_amount == "0" {
                            "0".to_string()
                        } else {
                            format!("{}{}", edit_amount, button_num)
                        }
                    }
                    _ => {
                        let button_num = if button_num == 11 { 0 } else { button_num };
                        let new_amount = format!("{}{}", edit_amount, button_num);

                        if self.enter_fiat {
                            format_fiat(&new_amount)
                        } else {
                            new_amount
                        }
                    }
                };

                if self.enter_fiat {
                    if let (Ok(amount), Some(sat_per_usd)) =
                        (new_amount.parse::<f64>(), self.sat_per_usd)
                    {
                        let value = (sat_per_usd as f64 * amount).round() as u64;
                        self.amount = value.to_string();
                    }

                    self.fiat_value = new_amount;
                } else {
                    if let (Ok(amount), Some(sat_per_usd)) =
                        (new_amount.parse::<u64>(), self.sat_per_usd)
                    {
                        let value = ((amount as f64 / sat_per_usd as f64) * 100.0).round() / 100.0;
                        self.fiat_value = format_fiat(&value.to_string());
                    }

                    self.amount = new_amount.trim_start_matches('0').to_string();
                }

                true
            }
            Msg::GotPrice(sats_per_dollar) => {
                log::debug!("{:?}", sats_per_dollar);

                self.sat_per_usd = Some(sats_per_dollar);
                true
            }
            Msg::EnterFiat => {
                self.enter_fiat = !self.enter_fiat;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_pubkey = ctx.link().callback(|_| Msg::AmountSubmitted);
        let l = (1..=12).collect::<Vec<_>>();

        let enter_fiat_cb = ctx.link().callback(|_| Msg::EnterFiat);
        html! {
                  <div class="flex justify-center">
                    <a class="block flex-1 p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-96 lg:max-w-lg sm:w-full">

        <div class="grid grid-rows-2 grid-flow-col gap-2">
            <div class="row-span-2 col-span-1 px-8 py-4"></div>
                    {
                        match self.enter_fiat {
                            true => {
                                html! {
                                <>
                                    <h1 class="row-span-1 col-span-8 text-3xl mb-4 font-light leadi text-center">{format!("${}", self.fiat_value.clone())}</h1>
                                    <h1 class="row-span-1 col-span-8 text-3xl mb-4 font-semibold leadi text-center">{format!("{} sats", self.amount.clone())}</h1>
                                </>
                                }

                            }
                            false => {
                                html! {
                                    <>
                                        <h1 class="row-span-1 col-span-8 text-3xl mb-4 font-semibold leadi text-center">{format!("{} sats", self.amount.clone())}</h1>
                                        <h1 class="row-span-1 col-span-8 text-3xl mb-4 font-light leadi text-center">{format!("${}", self.fiat_value.clone())}</h1>
                                    </>
                                }

                            }
                        }
                    }
                    <button class="row-span-2 col-span-1 px-8 py-4" onclick={enter_fiat_cb}><svg viewBox="0 0 24 24" stroke-width="1.5" fill="none" xmlns="http://www.w3.org/2000/svg" class="stroke-violet-400 h-1/2"><path d="M9.019 9A6.5 6.5 0 1115 14.981M8.5 22a6.5 6.5 0 110-13 6.5 6.5 0 010 13zM22 17a3 3 0 01-3 3h-2m0 0l2-2m-2 2l2 2M2 7a3 3 0 013-3h2m0 0L5 6m2-2L5 2" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path></svg></button>
                    </div>
                    <div class="grid grid-cols-3 gap-4">

                    {
                        l.into_iter().map(|i| {
                            let cb = ctx.link().callback(move |_| Msg::ButtonPressed(i ));
                            if i == 10 {
                                if self.enter_fiat {
                                    html!{<div></div>
                                }

                                } else {
                                html!{
                                    <>
                                    <div></div>
                                    </>
                                }
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
