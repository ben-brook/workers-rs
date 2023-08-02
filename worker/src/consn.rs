use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result as StdResult;

use js_sys::JsString;
use wasm_bindgen::{JsCast, JsValue};
pub use worker_sys::types::run;

use crate::env::EnvBinding;
use crate::Result;

pub use serde_wasm_bindgen;

// A Constellation Model.
pub struct ConsnModel(JsValue);

impl EnvBinding for ConsnModel {
    const TYPE_NAME: &'static str = "ConsnModel";

    fn get(val: JsValue) -> Result<Self> {
        let obj = js_sys::Object::from(val);
        if obj.constructor().name() == Self::TYPE_NAME || obj.constructor().name() == "Fetcher" {
            Ok(obj.unchecked_into())
        } else {
            Err(format!(
                "Binding cannot be cast to the type {} from {}",
                Self::TYPE_NAME,
                obj.constructor().name()
            )
            .into())
        }
    }
}

impl JsCast for ConsnModel {
    fn instanceof(val: &JsValue) -> bool {
        val.is_instance_of::<JsValue>()
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        Self(val.into())
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

impl From<ConsnModel> for JsValue {
    fn from(model: ConsnModel) -> Self {
        JsValue::from(model.0)
    }
}

impl AsRef<JsValue> for ConsnModel {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for ConsnModel {
    fn from(inner: JsValue) -> Self {
        Self(inner)
    }
}

#[derive(Clone)]
pub struct ConsnError {
    inner: js_sys::Error,
}

impl ConsnError {
    /// Gets the cause of the error specific to Constellation.
    pub fn cause(&self) -> String {
        if let Ok(cause) = self.inner.cause().dyn_into::<js_sys::Error>() {
            cause.message().into()
        } else {
            "unknown error".into()
        }
    }
}

impl std::fmt::Debug for ConsnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cause = self.inner.cause();

        f.debug_struct("ConsnError").field("cause", &cause).finish()
    }
}

impl Display for ConsnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cause = self.inner.cause();
        let cause = JsString::from(cause);
        write!(f, "{}", cause)
    }
}

impl AsRef<js_sys::Error> for ConsnError {
    fn as_ref(&self) -> &js_sys::Error {
        &self.inner
    }
}

impl AsRef<JsValue> for ConsnError {
    fn as_ref(&self) -> &JsValue {
        &self.inner
    }
}

fn cast_to_consn_error<T>(result: StdResult<T, JsValue>) -> StdResult<T, crate::Error> {
    let err = match result {
        Ok(value) => return Ok(value),
        Err(err) => err,
    };

    let err: JsValue = match err.dyn_into::<js_sys::Error>() {
        Ok(err) => {
            let message: String = err.message().into();

            // TODO: change?
            if message.starts_with("Constellation") {
                return Err(ConsnError { inner: err }.into());
            };
            err.into()
        }
        Err(err) => err,
    };

    Err(err.into())
}
