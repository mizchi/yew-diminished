//! This module contains provide dom bindings

// #![feature(use_extern_macros)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    pub type Node;
    #[wasm_bindgen(method, js_name = appendChild)]
    pub fn append_child(this: &Node, other: Node);
    #[wasm_bindgen(method, js_name = insertBefore)]
    pub fn insert_before(this: &Node, other: Node);
    #[wasm_bindgen(method, js_name = removeChild)]
    pub fn remove_child(this: &Node, other: Node);
    #[wasm_bindgen(method, getter, js_name = nextSibling)]
    pub fn next_sibling(this: &Node);
    #[wasm_bindgen(method, getter, js_name = firstChild)]
    pub fn first_child(this: &Node);
    #[wasm_bindgen(method, getter, js_name = lastChild)]
    pub fn last_child(this: &Node);
    #[wasm_bindgen(method, js_name = setAttribute)]
    pub fn set_attribute(this: &Node, attr: &str, value: &str);
    #[wasm_bindgen(method, js_name = removeAttribute)]
    pub fn remove_attribute(this: &Node, attr: &str);

    #[wasm_bindgen(extends = Node)]
    pub type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    pub fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, getter)]
    pub fn inner_html(this: &Element) -> JsValue;

    #[wasm_bindgen(extends = Node)]
    pub type Text;
    #[wasm_bindgen(method, getter, js_name = nodeValue)]
    pub fn node_value(this: &Text) -> JsValue;
    #[wasm_bindgen(method, setter = innerHTML)]
    pub fn set_node_value(this: &Text, text: &str);

    #[wasm_bindgen(extends = Element)]
    pub type HTMLDocument;
    #[wasm_bindgen(method, js_name = createElement)]
    pub fn create_element(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, js_name = createTextNode)]
    pub fn create_text_node(this: &HTMLDocument, text: &str) -> Text;
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &HTMLDocument) -> Element;

    pub static document: HTMLDocument;
}
