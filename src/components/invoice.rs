use cashu_crab::{Amount, Invoice};
use qrcode::render::svg;
use qrcode::QrCode;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub invoice: Invoice,
    pub fiat_value: String,
    pub home_cb: Callback<MouseEvent>,
}

#[derive(Default)]
pub struct InvoiceView {
    amount: Amount,
    invoice_qr: VNode,
}

impl Component for InvoiceView {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let qr_svg = QrCode::new(ctx.props().invoice.to_string().as_bytes())
            .unwrap()
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        // escapes the string to make it html
        let invoice_qr_svg = Html::from_html_unchecked(AttrValue::from(qr_svg));

        let amount = ctx.props().invoice.amount_milli_satoshis().unwrap_or(0);

        let amount = Amount::from_msat(amount);

        Self {
            invoice_qr: invoice_qr_svg,
            amount,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>


        <div class="flex items-center justify-center">
          <a class="block p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
            <h1 class="text-3xl mb-4 font-semibold leadi text-center">{format!("{} sats", self.amount.to_sat())}</h1>
            <h1 class="text-3xl mb-4 font-light leadi text-center">{format!("{} USD", ctx.props().fiat_value.clone())}</h1>
            <div class="flex flex-col items-center">

                { self.invoice_qr.clone() }

                <p class="flex-1 dark:text-gray-400" style="max-width: 33vw; word-wrap: break-word;">{ctx.props().invoice.to_string() }</p>
                <button class="block w-full text-5xl lg:text-xl p-6 my-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().home_cb.clone()}>{"Cancel"}</button>

            </div>
          </a>
        </div>
            </>
        }
    }
}
