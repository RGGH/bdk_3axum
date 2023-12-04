use rstml_component::{move_html, write_html, For, HtmlComponent, HtmlContent};
use rstml_component_axum::HtmlContentAxiosExt;

use env_logger::Env;
use log::info;
use log::LevelFilter;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

use crate::web;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::body::Body;
use axum::extract::Query;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::keys::{
    bip39::{Language, Mnemonic, WordCount},
    DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
};
use bdk::template::Bip84;
use bdk::wallet::AddressIndex;
use bdk::{miniscript, KeychainKind, Wallet};

// Define the constant for the configuration file path
const CONFIG_PATH: &str = "wallet_config.json";

fn read_file_content(file_path: &str) -> io::Result<String> {
    // Open the file in read-only mode
    let mut config_file = File::open(file_path)?;

    // Create a buffer to store the file content
    let mut config_content = String::new();

    // Read the contents of the file into the buffer
    config_file.read_to_string(&mut config_content)?;

    Ok(config_content)
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/gen_wallet", get(gen_wallet))
        .route("/api/load_wallet", get(load_wallet))
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

#[derive(Debug, Serialize, Deserialize)]
struct WalletConfig {
    xprv: String,
    network: Network,
}

async fn gen_wallet(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
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

    let new_wallet = wallet.get_address(AddressIndex::New).unwrap();
    // get a new address (this increments revealed derivation index)
    println!("revealed address: {:?}", new_wallet.address);

    let amount: Option<&String> = params.get("amount");

    Json(json!({
        "message": new_wallet.address,
        "amount": amount,
    }))
}

async fn load_wallet(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    // Read the file content and handle the result
    let config_content_result = read_file_content(CONFIG_PATH);

    // Check if reading the file was successful
    let config_content = match config_content_result {
        Ok(content) => content,
        Err(err) => {
            // Handle the error, e.g., print an error message and return an error response
            eprintln!("Error reading config file: {}", err);
            return Json(json!({
                "error": "Failed to read configuration file",
            }));
        }
    };

    // Log xprv_str just before parsing
    let wallet_config: WalletConfig = serde_json::from_str(&config_content).unwrap();

    // Extract values from the configuration
    let xprv = &wallet_config.xprv;
    let network = wallet_config.network;

    info!("xprv: {:?}", &xprv);

    // Parse xprv
    let xprv = ExtendedPrivKey::from_str(&xprv)
        // .map_err(|err| {
        //     eprintln!("Error parsing xprv: {}", err);
        //     Json(json!({
        //         "error": "Failed to parse xprv",
        //     }))
        // })
        .unwrap();

    // Get xprv from the extended key
    // let xprv = xkey.into_xprv(network).unwrap();
    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let wallet = Wallet::new(
        Bip84(xprv, KeychainKind::External),
        Some(Bip84(xprv, KeychainKind::Internal)),
        network,
        MemoryDatabase::default(),
    )
    .unwrap();

    let new_wallet = wallet.get_address(AddressIndex::New).unwrap();
    // get a new address (this increments revealed derivation index)
    println!("revealed (loaded) address: {:?}", new_wallet.address);

    let amount: Option<&String> = params.get("amount");

    Json(json!({
        "message": new_wallet.address,
        "amount": amount,
    }))
}
