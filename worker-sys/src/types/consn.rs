use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=::js_sys::Object, js_name=Tensor)]
    #[derive(Debug, Clone)]
    pub type Tensor;

    #[wasm_bindgen(constructor)]
    pub fn new(
        tensorType: &str,
        shape: Vec<u32>,
        value: Vec<JsValue>,
        name: Option<String>,
    ) -> Tensor;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn run(binding: JsValue, modelId: &str, inputs: Vec<Tensor>) -> JsValue;
}
