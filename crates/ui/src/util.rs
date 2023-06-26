use wasm_bindgen::prelude::*;

/// Initiates a download in the browser of the given filename and contents.
/// To do so, a temporary element is created and clicked.
pub fn download_string(filename: &str, contents: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let encoded = js_sys::encode_uri_component(contents);

    // create a temporary element to hold the download link
    let element = document.create_element("a")?;
    let element = element.dyn_into::<web_sys::HtmlElement>()?;
    element.set_attribute("href", &format!("data:text/plain;charset=utf-8,{encoded}"))?;
    element.set_attribute("download", filename)?;
    element.style().set_property("display", "none")?;

    // add the element to the document and click it
    document.body().unwrap().append_child(&element)?;
    element.click();
    document.body().unwrap().remove_child(&element)?;

    Ok(())
}
