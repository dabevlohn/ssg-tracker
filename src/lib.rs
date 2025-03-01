use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    custom_claim: String,
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
}

#[wasm_bindgen]
pub fn validate_token(token: &str) -> String {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    //validation.required_spec_claims = HashSet::new();
    //validation.validate_aud = false;
    // config to validate the aud
    validation.set_audience(&vec!["my@ema.il"]);

    let secret = jsonwebtoken::DecodingKey::from_secret("ecpkdocs".as_bytes());
    // decode token
    let res = jsonwebtoken::decode::<Claims>(&token, &secret, &validation);
    //log(&res.claims.custom_claim);
    log(&format!("Token: {:?}", res));
    format!("{:?}", res.unwrap().claims)
}

#[wasm_bindgen]
pub fn create_token(
    current_location_origin: &str,
    current_location_path: &str,
    ident_header: &str,
    ident_secret: &str,
) -> String {
    // create the claim
    let c = Claims {
        custom_claim: ident_secret.to_string(),
        iss: current_location_origin.to_string(),
        sub: current_location_path.to_string(),
        aud: ident_header.to_string(),
        exp: 9999999999,
    };
    // create the header
    let header = jsonwebtoken::Header::default();
    // create the encoding key using the secret string
    let secret = jsonwebtoken::EncodingKey::from_secret("ecpkdocs".as_bytes());
    // encode token
    let token = jsonwebtoken::encode(&header, &c, &secret).unwrap();
    log(&format!(
        "{}{}?jwt={}",
        current_location_origin, current_location_path, token
    ));
    token
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
