use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{IdbDatabase, IdbTransactionMode};

const DB_NAME: &str = "alexo-io";
const STORE_NAME: &str = "preferences";
const THEME_KEY: &str = "theme";

async fn await_request(request: &web_sys::IdbRequest) -> Option<JsValue> {
    let request = request.clone();
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        request.set_onsuccess(Some(
            Closure::once_into_js(move || {
                let _ = resolve.call0(&JsValue::NULL);
            })
            .unchecked_ref(),
        ));
        request.set_onerror(Some(
            Closure::once_into_js(move || {
                let _ = reject.call0(&JsValue::NULL);
            })
            .unchecked_ref(),
        ));
    });
    JsFuture::from(promise).await.ok()?;
    request.result().ok()
}

async fn open_db() -> Option<IdbDatabase> {
    let idb = web_sys::window()?.indexed_db().ok()??;
    let request = idb.open_with_u32(DB_NAME, 1).ok()?;

    let request_clone = request.clone();
    let onupgrade = Closure::once_into_js(move || {
        if let Ok(result) = request_clone.result() {
            let db: IdbDatabase = result.unchecked_into();
            if !db.object_store_names().contains(STORE_NAME) {
                let _ = db.create_object_store(STORE_NAME);
            }
        }
    });
    request.set_onupgradeneeded(Some(onupgrade.unchecked_ref()));

    await_request(&request).await.map(|v| v.unchecked_into())
}

pub async fn load_theme() -> Option<crate::Theme> {
    let db = open_db().await?;
    let tx = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readonly)
        .ok()?;
    let store = tx.object_store(STORE_NAME).ok()?;
    let request = store.get(&JsValue::from_str(THEME_KEY)).ok()?;
    let value = await_request(&request).await?;
    value.as_string().and_then(|s| match s.as_str() {
        "dark" => Some(crate::Theme::Dark),
        "light" => Some(crate::Theme::Light),
        _ => None,
    })
}

pub async fn save_theme(theme: crate::Theme) {
    let Some(db) = open_db().await else { return };
    let Ok(tx) = db.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite) else {
        return;
    };
    let Ok(store) = tx.object_store(STORE_NAME) else {
        return;
    };
    let value = match theme {
        crate::Theme::Dark => "dark",
        crate::Theme::Light => "light",
    };
    let _ = store.put_with_key(&JsValue::from_str(value), &JsValue::from_str(THEME_KEY));
}
