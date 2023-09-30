use crate::{constants, error::Result};
use rexie::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub(crate) async fn build_database() -> Result<Rexie> {
    let rexie = Rexie::builder(constants::APP_NAME)
        // The version number is used to determine whether the database needs to be upgraded.
        // If the version number is higher than the current version, the database will be upgraded.
        .version(2)
        .add_object_store(ObjectStore::new(constants::KEY_VALUE_STORE))
        .build()
        .await?;

    Ok(rexie)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl From<KeyValue> for JsValue {
    fn from(kv: KeyValue) -> Self {
        serde_wasm_bindgen::to_value(&kv).unwrap()
    }
}

impl From<JsValue> for KeyValue {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}

/// Assigns a value in the key-value store.
pub(crate) async fn set_key(rexie: &Rexie, key: &str, value: &str) -> Result<()> {
    let transaction =
        rexie.transaction(&[constants::KEY_VALUE_STORE], TransactionMode::ReadWrite)?;

    let store = transaction.store(constants::KEY_VALUE_STORE)?;

    let item = serde_wasm_bindgen::to_value(value)?;
    let key = JsValue::from_str(key);

    store.put(&item, Some(&key)).await?;

    transaction.done().await?;

    Ok(())
}

/// Retrieves a value from the key-value store.
pub(crate) async fn get_key(rexie: &Rexie, key: &str) -> Result<Option<String>> {
    let transaction =
        rexie.transaction(&[constants::KEY_VALUE_STORE], TransactionMode::ReadOnly)?;

    let store = transaction.store(constants::KEY_VALUE_STORE)?;

    let result = store.get(&key.into()).await?;

    transaction.done().await?;

    if result.is_undefined() {
        return Ok(None);
    }

    let result = result.as_string().expect("value is not a string");
    Ok(Some(result))
}

/// Removes a value from the key-value store.
pub(crate) async fn remove_key(rexie: &Rexie, key: &str) -> Result<()> {
    let transaction =
        rexie.transaction(&[constants::KEY_VALUE_STORE], TransactionMode::ReadWrite)?;

    let store = transaction.store(constants::KEY_VALUE_STORE)?;
    store.delete(&key.into()).await?;
    transaction.done().await?;

    Ok(())
}
