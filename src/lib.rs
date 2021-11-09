use wasm_bindgen::prelude::*;
use js_sys::Promise;

use std::time::Duration;

use fluvio_wasm_timer::Delay;

pub async fn sleep(s: i32) {
    let s_u64 = u64::try_from(s).unwrap();
            // It panics when the i32 value is outside of the range of u64.
    let d = Duration::from_secs(s_u64);

    Delay::new(d).await.unwrap();
}

#[wasm_bindgen]
pub async fn js_sleep(s: i32) -> Promise {
    sleep(s).await;

    Promise::resolve(&JsValue::NULL)
}
