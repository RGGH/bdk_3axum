use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use std::collections::HashMap;

use crate::web;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::body::Body;
use axum::extract::Query;
use axum::http::{Response, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use axum::{Extension, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use bdk::keys::{
    bip39::{Language, Mnemonic, WordCount},
    DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
};
use bdk::template::Bip84;
use bdk::wallet::AddressIndex;
use bdk::{miniscript, KeychainKind, Wallet};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/wallet", get(gwallet))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->>{:<12} - api login", "HANDLER");
    // Err Message
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // success body
    let body = Json(json!(
                {"result" : {
                    "success" : true
                }
                }
    ));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

// Handler for "/api/wallet" route
async fn gwallet(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let network = Network::Testnet;
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic_words = mnemonic.to_string();
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    // Generate the extended key
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    // Get xprv from the extended key
    let xprv = xkey.into_xprv(network).unwrap();
    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let wallet = Wallet::new(
        Bip84(xprv, KeychainKind::External),
        Some(Bip84(xprv, KeychainKind::Internal)),
        network,
        MemoryDatabase::default(),
    )
    .unwrap();

    // get a new address (this increments revealed derivation index)
    println!(
        "revealed address: {:?}",
        wallet.get_address(AddressIndex::New)
    );

    let q: Option<&String> = params.get("q");

    Json(json!({
        "message": "Hello!",
        "q": q,

    }))
}
