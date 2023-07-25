use std::time::Duration;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use cashu_crab::{
    nuts::{nut00::wallet::Token, nut03::RequestMintResponse},
    wallet::Wallet,
    Amount, Client as CashuClient, Invoice,
};
use gloo::storage::LocalStorage;
use gloo::storage::Storage;
use gloo::timers::future::sleep;
use log::warn;
use nostr_sdk::{prelude::FromPkStr, Client, Keys};
use tokio::sync::Mutex as TokioMutex;
use url::Url;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::{
    invoice::InvoiceView, invoice_paid::InvoicePaid, pos::Pos, set_mint::SetMint,
    set_rec_key::SetRecKey, set_relays::SetRelays, settings::Settings,
};
use crate::utls;

pub const NOSTR_KEY: &str = "nostr_rec";
pub const MINT_URL_KEY: &str = "mint_url";
pub const RELAYS_KEY: &str = "relays";

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    SetMint,
    SetRecKey,
    Pos,
    Invoice((Invoice, String)),
    InvoicePaid,
    SetRelays,
    Settings,
}

pub enum Msg {
    NostrRecKeySet(Keys),
    MintUrlSet(Url),
    ClientCreated(Client),
    WalletCreated(Wallet),
    AmountSet((Amount, String)),
    InvoiceSet((Amount, String, RequestMintResponse)),
    InvoicePaid((Amount, Token)),
    AddRelay(Url),
    RelaysSet,
    Home,
    Settings,
    AddRelayView,
    SetMintView,
    SetPubkeyView,
}

#[derive(Debug, Default, Clone)]
pub struct App {
    view: View,
    nostr_receice_pubkey: Option<Keys>,
    relays: HashSet<Url>,
    wallet: Arc<Mutex<Option<Wallet>>>,
    nostr_client: Arc<TokioMutex<Option<Client>>>,
    unpaid_invoices: HashSet<String>,
}

// Creates the websocket client that is used for communicating with relays
async fn create_client(keys: &Keys, relays: Vec<Url>, client_cb: Callback<Client>) -> Result<()> {
    let client = Client::new(keys);
    let r: Vec<String> = relays.into_iter().map(|url| url.to_string()).collect();
    client.add_relays(r).await?;
    client.connect().await;
    client_cb.emit(client);
    Ok(())
}

async fn create_wallet(mint_url: &Url, wallet_cb: Callback<Wallet>) -> Result<()> {
    let client = CashuClient::new(mint_url.as_str())?;
    let mint_keys = client.get_keys().await?;
    let wallet = Wallet::new(client, mint_keys);

    wallet_cb.emit(wallet);
    Ok(())
}

impl App {
    fn app_view(&self) -> View {
        let wallet = self.wallet.lock().unwrap().clone();
        let key = self.nostr_receice_pubkey.clone();

        log::debug!("{:?}", self.relays);

        match (key, wallet, self.relays.is_empty()) {
            (Some(_), Some(_), false) => View::Pos,
            (None, Some(_), _) => View::SetRecKey,
            (Some(_), None, _) => View::SetMint,
            (None, None, _) => View::SetMint,
            (Some(_), Some(_), true) => View::SetRelays,
        }
    }

    async fn get_invoice(
        &self,
        amount: Amount,
        fiat_value: String,
        invoice_cb: Callback<(Amount, String, RequestMintResponse)>,
    ) -> Result<()> {
        let wallet = self.wallet.lock().unwrap().clone();

        if let Some(wallet) = wallet {
            let invoice = wallet.request_mint(amount).await?;

            invoice_cb.emit((amount, fiat_value, invoice))
        }

        Ok(())
    }

    async fn mint(
        &mut self,
        amount: Amount,
        hash: String,
        mint_cb: Callback<(Amount, Token)>,
    ) -> Result<()> {
        let wallet = self.wallet.lock().unwrap().clone();

        if let Some(wallet) = wallet {
            loop {
                if let Ok(proofs) = wallet.mint(amount, &hash).await {
                    let token = Token::new(wallet.client.mint_url, proofs, None);
                    self.unpaid_invoices.remove(&hash);

                    mint_cb.emit((amount, token));
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }
        }

        Ok(())
    }

    async fn send_token(&self, token: Token) -> Result<()> {
        if let (Some(nostr_client), Some(nostr_rec)) = (
            self.nostr_client.lock().await.clone(),
            self.nostr_receice_pubkey.clone(),
        ) {
            let _ = nostr_client
                .send_direct_msg(nostr_rec.public_key(), token.convert_to_string()?)
                .await;
        }

        Ok(())
    }

    async fn add_relay(&self, relay: Url) -> Result<()> {
        log::debug!("i{:?}", relay);
        if let Some(nostr_client) = self.nostr_client.lock().await.clone() {
            log::debug!("client some");
            nostr_client.add_relay(&relay.to_string()).await?;
            nostr_client.connect().await;
        }
        Ok(())
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mint_url: Option<Url> = LocalStorage::get::<Url>(MINT_URL_KEY).ok();

        let nostr_rec_key: Option<Keys> = LocalStorage::get::<String>(NOSTR_KEY)
            .ok()
            .and_then(|u| serde_json::from_str(u.as_str()).map(|k: String| k).ok())
            .and_then(|u| Keys::from_pk_str(&u).ok());

        let relays: HashSet<Url> = LocalStorage::get(RELAYS_KEY).unwrap_or_default();

        let relays_vec = relays.iter().cloned().collect();

        match (mint_url, nostr_rec_key) {
            (Some(url), Some(pubkey)) => {
                let keys = utls::handle_keys(None).unwrap();

                let client_cb = ctx.link().callback(Msg::ClientCreated);
                let wallet_cb = ctx.link().callback(Msg::WalletCreated);

                spawn_local(async move {
                    // TODO: Set relays

                    create_client(&keys, relays_vec, client_cb).await.unwrap();
                    create_wallet(&url, wallet_cb).await.unwrap();
                });

                Self {
                    view: View::Pos,
                    nostr_receice_pubkey: Some(pubkey),
                    relays,
                    ..Default::default()
                }
            }
            // Mint Url is not set
            (None, None) => Self {
                relays,
                ..Default::default()
            },
            // Mint url is set but user not logged in
            (Some(url), None) => {
                let wallet_cb = ctx.link().callback(Msg::WalletCreated);

                spawn_local(async move {
                    create_wallet(&url, wallet_cb).await.unwrap();
                });

                Self {
                    view: View::SetRecKey,
                    relays,
                    ..Default::default()
                }
            }
            (None, Some(pubkey)) => Self {
                nostr_receice_pubkey: Some(pubkey),
                view: View::SetMint,
                relays,
                ..Default::default()
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClientCreated(client) => {
                self.nostr_client = Arc::new(TokioMutex::new(Some(client)));
                self.view = self.app_view();
                true
            }
            Msg::WalletCreated(wallet) => {
                self.wallet = Arc::new(Mutex::new(Some(wallet)));
                self.view = self.app_view();
                true
            }
            Msg::NostrRecKeySet(rec_key) => {
                LocalStorage::set(
                    NOSTR_KEY,
                    serde_json::to_string(&rec_key.public_key()).unwrap(),
                )
                .ok();

                self.nostr_receice_pubkey = Some(rec_key);
                self.view = self.app_view();

                true
            }
            Msg::MintUrlSet(url) => {
                let url_clone = url.clone();
                LocalStorage::set(MINT_URL_KEY, url).ok();

                let create_wallet_cb = ctx.link().callback(Msg::WalletCreated);

                spawn_local(async move {
                    if let Err(err) = create_wallet(&url_clone, create_wallet_cb).await {
                        warn!("Could not create wallet {:?}", err);
                    }
                });
                true
            }
            Msg::AmountSet((amount, fiat_value)) => {
                let get_invoice_cb = ctx.link().callback(Msg::InvoiceSet);
                let app = self.clone();
                spawn_local(async move {
                    if let Err(err) = app.get_invoice(amount, fiat_value, get_invoice_cb).await {
                        warn!("Could not create wallet {:?}", err);
                    }
                });
                true
            }
            Msg::InvoiceSet((amount, fiat_value, invoice_response)) => {
                self.view = View::Invoice((invoice_response.pr, fiat_value));
                self.unpaid_invoices.insert(invoice_response.hash.clone());

                let invoice_paid_cb = ctx.link().callback(Msg::InvoicePaid);
                let mut app = self.clone();
                spawn_local(async move {
                    if let Err(err) = app
                        .mint(amount, invoice_response.hash, invoice_paid_cb)
                        .await
                    {
                        warn!("Could not create wallet {:?}", err);
                    }
                });
                true
            }
            Msg::InvoicePaid((_amount, token)) => {
                let app = self.clone();
                self.view = View::InvoicePaid;
                spawn_local(async move {
                    let _ = app.send_token(token).await;
                });

                true
            }
            Msg::Home => {
                self.view = self.app_view();

                true
            }
            Msg::AddRelay(relay) => {
                log::debug!("Msg: {:?}", relay);
                let app = self.clone();
                let relay_clone = relay.clone();
                spawn_local(async move {
                    let _ = app.add_relay(relay).await;
                });
                self.relays.insert(relay_clone);

                log::debug!("relays: {:?}", self.relays);
                LocalStorage::set(RELAYS_KEY, self.relays.clone()).ok();

                true
            }
            Msg::RelaysSet => {
                self.view = self.app_view();

                true
            }
            Msg::Settings => {
                self.view = View::Settings;
                true
            }
            Msg::SetPubkeyView => {
                self.view = View::SetRecKey;
                true
            }
            Msg::SetMintView => {
                self.view = View::SetMint;
                true
            }
            Msg::AddRelayView => {
                self.view = View::SetRelays;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("{:?}", self.view);
        html! {
            <main>

                    {

                match &self.view {
                    View::Pos => {
                        let amount_cb = ctx.link().callback(Msg::AmountSet);
                        let settings_cb = ctx.link().callback(|_| Msg::Settings);

                        html!{
                            <>
                            <div style="display: flex; justify-content: flex-end;">
                              <button class="px-6 py-2 mt-2 rounded-sm shadow-sm dark:bg-violet-400 dark:text-gray-900" onclick={settings_cb.clone()}>{"Settings"}</button>
                            </div>
                            <Pos {amount_cb} />
                            </>
                        }

                    }
                    View::SetMint => {
                        let mint_set_cb = ctx.link().callback(Msg::MintUrlSet);

                        html! {
                            <>
                             <SetMint {mint_set_cb} />
                            </>
                        }
                    }
                    View::SetRecKey => {
                        let set_rec_key = ctx.link().callback(Msg::NostrRecKeySet);

                        html!{
                        <>
                            <SetRecKey logged_in_callback={set_rec_key}/>
                        </>
                        }
                    }
                    View::Invoice((invoice, fiat_value)) => {
                        let home_cb = ctx.link().callback(|_| Msg::Home);
                        html!{
                            <InvoiceView invoice={invoice.clone()} fiat_value={fiat_value.clone()} {home_cb} />
                        }
                    }
                    View::InvoicePaid => {
                        let home_cb = ctx.link().callback(|_| Msg::Home);
                        html!{
                            <InvoicePaid {home_cb} />
                        }
                    }
                    View::SetRelays => {
                        let relays_set_cb = ctx.link().callback(|_| Msg::RelaysSet);
                        let add_relay_cb = ctx.link().callback(Msg::AddRelay);

                        html!{
                            <SetRelays {add_relay_cb} {relays_set_cb} />
                        }
                    }
                    View::Settings => {
                        let add_relay_cb = ctx.link().callback(|_| Msg::AddRelayView);
                        let set_pubkey_cb = ctx.link().callback(|_| Msg::SetPubkeyView);
                        let set_mint_cb = ctx.link().callback(|_| Msg::SetMintView);
                        let home_cb = ctx.link().callback(|_| Msg::Home);

                        html! {
                            <Settings {add_relay_cb} {set_pubkey_cb} {set_mint_cb} {home_cb} />
                        }

                    }
                }
        }

        </main>
        }
    }
}
