use cashu_crab::Invoice;
use qrcode::render::svg;
use qrcode::QrCode;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub invoice: Invoice,
}

#[derive(Default)]
pub struct InvoiceView {
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

        Self {
            invoice_qr: invoice_qr_svg,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>


        <div class="flex items-center justify-center">
          <a class="block p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
            <div class="flex flex-col items-center">

                 { self.invoice_qr.clone() }

                    <p class="flex-1 dark:text-gray-400" style="max-width: 33vw; word-wrap: break-word;">{ctx.props().invoice.to_string() }</p>

            </div>
          </a>
        </div>
            </>
        }
    }
}
