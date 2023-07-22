//! Nostr utils
// Modified from https://github.com/0xtrr/nostr-tool
// Copyright (c) 2022 0xtr
// Distributed under the MIT software license

use anyhow::Result;
use log::info;

use nostr_sdk::key::FromSkStr;
use nostr_sdk::Keys;

pub fn handle_keys(private_key: Option<String>) -> Result<Keys> {
    // Parse and validate private key
    let keys = match private_key {
        Some(pk) => {
            // create a new identity using the provided private key
            Keys::from_sk_str(pk.as_str())?
        }
        None => {
            // create a new identity with a new keypair
            info!("No private key provided, creating new identity");
            Keys::generate()
        }
    };

    Ok(keys)
}
