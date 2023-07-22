use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub home_cb: Callback<MouseEvent>,
}

#[derive(Default)]
pub struct InvoicePaid;

impl Component for InvoicePaid {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="flex items-center justify-center">
          <a class="block p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
            <div class="flex flex-col items-center">
              <div class="w-1/2 text-green-500 mb-2">
                <svg width="100%" height="100%" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M7 12.5l3 3 7-7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                  <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
              </div>

              <button class="px-6 py-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={ctx.props().home_cb.clone()}>{"Home"}</button>
            </div>
          </a>
        </div>



                                }
    }
}
