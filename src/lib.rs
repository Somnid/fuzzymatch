extern crate serde_derive;
extern crate wasm_bindgen;

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod fuzzymatch;

#[derive(Serialize, Deserialize, Debug)]
struct MatchIndex(usize, String);

#[wasm_bindgen]
pub fn fuzzymatch(search_keys: JsValue, term: String, threshold: f32) -> JsValue {
    let search_keys_unwrapped = search_keys.into_serde::<Vec<String>>().unwrap();
    let search_keys_internal: Vec<&str> = search_keys_unwrapped
                            .iter()
                            .map(|k| &k[..])
                            .collect();
    let match_indicies = fuzzymatch::fuzzymatch(&search_keys_internal, &term, threshold);
    let wasm_match_indicies: Vec<MatchIndex> = match_indicies.iter().map(|i| MatchIndex(i.0, i.1.to_string())).collect();
    JsValue::from_serde(&wasm_match_indicies).unwrap()
}