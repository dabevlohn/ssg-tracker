use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    iss: String,
    sub: String,
    aud: String,
    custom_claim: String,
    exp: u64,
}

const SECRET: &str = "ecpkdocs";
const AUDIENCE: &str = "docportal";

#[wasm_bindgen]
pub fn validate_token(token: &str) -> String {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_audience(&vec![AUDIENCE]);
    //validation.required_spec_claims = HashSet::new();
    //validation.validate_aud = false;

    let secret = jsonwebtoken::DecodingKey::from_secret(SECRET.as_bytes());
    let res = jsonwebtoken::decode::<Claims>(&token, &secret, &validation);
    // log(&format!("Token: {:?}", res));
    format!("{:?}", res.unwrap().claims)
}

#[wasm_bindgen]
pub fn create_token() -> String {
    let mut xemail: String = getXemailHeader();
    if xemail.is_empty() {
        xemail = "None".to_string()
    }

    let mut current_origin = getCurrentOrigin();
    if current_origin.is_empty() {
        current_origin = "None".to_string();
    }

    let mut current_pathname = getCurrentPathname();
    if current_pathname.is_empty() {
        current_pathname = "None".to_string();
    }

    let c = Claims {
        iss: current_origin.to_string(),
        sub: current_pathname.to_string(),
        aud: AUDIENCE.to_string(),
        custom_claim: xemail.to_string(),
        exp: 9999999999,
    };
    let header = jsonwebtoken::Header::default();
    let secret = jsonwebtoken::EncodingKey::from_secret(SECRET.as_bytes());
    let token = jsonwebtoken::encode(&header, &c, &secret).unwrap();
    let track_url = format!("{}{}?jwt={}", current_origin, current_pathname, token);
    headRequest(&track_url, "");
    token
}

#[wasm_bindgen]
extern "C" {
    pub fn getCurrentOrigin() -> String;
    pub fn getCurrentPathname() -> String;
    pub fn getXemailHeader() -> String;
    pub fn headRequest(url: &str, callback: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
