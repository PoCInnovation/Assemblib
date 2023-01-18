// use std::convert::TryInto;
use std::f64;
use web_sys::Element;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use web_sys::{HtmlElement, Document, HtmlCanvasElement};



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, poc-test-wasm!");
}

#[wasm_bindgen]
pub fn display_blue_square() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    svg.set_attribute("width", "100")?;
    svg.set_attribute("hight", "100")?;
    svg.set_attribute("viewBox", "0 0 100 100")?;

    let square = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
    square.set_attribute("width", "100")?;
    square.set_attribute("height", "100")?;
    square.set_attribute("fill", "blue")?;
    svg.append_child(&square)?;
    body.append_child(&svg)?;

    Ok(())
}

fn attributes_to_hashmap(attributes_str: &str) -> HashMap<String, &str> {
    let split = attributes_str.split(";");
    let mut attributes_map = HashMap::new();

    let attributes_list: Vec<&str> = split.collect();

    for content in attributes_list {
        let key_value_list = content.split("=");

        let key_value: Vec<&str> = key_value_list.collect();

        attributes_map.insert(String::from(key_value[0]), key_value[1]);
    }
    return attributes_map;
}

fn create_svg_element(document: &Document, elem_type: &str, attributes: HashMap<String, &str>) -> Element {
    let element = document.create_element_ns(Some("http://www.w3.org/2000/svg"),elem_type).unwrap();

    for (key, value) in &attributes {
        element.set_attribute(key, value).unwrap();
    }

    return element;
}

#[wasm_bindgen]
pub fn display_rocket() -> Result<(), JsValue> {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");

    let figure = document.create_element("figure")?;
    let div = document.create_element("div").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
    div.set_attribute("class","rocket-main")?;

    let svg = create_svg_element(&document, "svg", attributes_to_hashmap("width=512px;height=512px;viewBox=0 0 416.449 416.449;id=Layer_1"));

    let base_g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    base_g.set_attribute("id","_x31_5._Rocket_2_")?;

    let g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;
    g.set_attribute("id","XMLID_65_")?;

    let g1 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let g1a = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1a = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;

    path1a.set_attribute("style", "fill:#FF7124;")?;
    path1a.set_attribute("d", "M399.76,16.699c10.12,37.84,8.67,78.13-4.34,115.28h-0.01L284.48,21.049v-0.01      C321.63,8.029,361.92,6.579,399.76,16.699z")?;

    g1a.append_child(&path1a)?;
    g1.append_child(&g1a)?;

    let g1b = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1b = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1b.set_attribute("style", "fill:#F2D59F;")?;
    path1b.set_attribute("d", "M90.21,207.929l87.14-101.42h0.01l33.71-39.24c21.43-21.43,46.6-36.84,73.41-46.23v0.01      l110.93,110.93h0.01c-9.39,26.81-24.8,51.98-46.23,73.41l-39.24,33.71l-101.43,87.14l-29.57-29.57l-29.58-29.58l-29.58-29.58      L90.21,207.929z M296.11,193.399c20.18-20.17,20.18-52.89,0-73.06c-20.17-20.18-52.89-20.18-73.06,0      c-20.18,20.17-20.18,52.89,0,73.06C243.22,213.579,275.94,213.579,296.11,193.399z")?;

    g1b.append_child(&path1b)?;
    g1.append_child(&g1b)?;

    let g1c = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1c = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1c.set_attribute("style", "fill:#F2D59F;")?;
    path1c.set_attribute("d", "M309.95,239.099c1.74,45.6-14.8,91.78-49.61,126.59c-10.69,10.68-22.44,19.65-34.93,26.89      l-16.89-66.34L309.95,239.099z")?;

    g1c.append_child(&path1c)?;
    g1.append_child(&g1c)?;

    let g1d = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1d = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1d.set_attribute("style", "fill:#8ECAC1;")?;
    path1d.set_attribute("d", "M296.11,120.339c20.18,20.17,20.18,52.89,0,73.06c-20.17,20.18-52.89,20.18-73.06,0      c-20.18-20.17-20.18-52.89,0-73.06C243.22,100.159,275.94,100.159,296.11,120.339z")?;

    g1d.append_child(&path1d)?;
    g1.append_child(&g1d)?;

    let g1e = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1e = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1e.set_attribute("style", "fill:#E6B263;")?;
    path1e.set_attribute("d", "M208.52,326.239l-39.94,14.71c-10.98,4.05-23.31,1.34-31.58-6.94l-6.85-6.85l48.8-30.49      L208.52,326.239z")?;

    g1e.append_child(&path1e)?;
    g1.append_child(&g1e)?;


    let g1f = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1f = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1f.set_attribute("style", "fill:#E6B263;")?;
    path1f.set_attribute("points", "178.95,296.669 130.15,327.159 130.14,327.159 109.72,306.739 149.37,267.089     ")?;

    g1f.append_child(&path1f)?;
    g1.append_child(&g1f)?;

    let g1g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1g.set_attribute("style", "fill:#F2D59F;")?;
    path1g.set_attribute("d", "M177.35,106.509l-87.14,101.42l-66.33-16.88c7.24-12.49,16.21-24.24,26.89-34.93      C85.58,121.309,131.74,104.769,177.35,106.509z")?;

    g1g.append_child(&path1g)?;
    g1.append_child(&g1g)?;

    let g1h = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1h = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
    path1h.set_attribute("style", "fill:#E6B263;")?;
    path1h.set_attribute("points", "149.37,267.089 109.72,306.739 89.3,286.309 119.79,237.509     ")?;

    g1h.append_child(&path1h)?;
    g1.append_child(&g1h)?;

    let g1i = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path1i = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path1i.set_attribute("style", "fill:#E6B263;")?;
    path1i.set_attribute("d", "M119.79,237.509l-30.49,48.8l-6.86-6.85c-8.27-8.28-10.98-20.6-6.94-31.58l14.71-39.95      L119.79,237.509z")?;

    g1i.append_child(&path1i)?;
    g1.append_child(&g1i)?;
    g.append_child(&g1)?;



    let g2 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let g2a = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2a = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2a.set_attribute("style", "fill:#5E2A41;")?;
    path2a.set_attribute("d", "M28.88,339.459c-2.559,0-5.119-0.977-7.071-2.929c-3.905-3.905-3.905-10.237,0-14.143      l20.54-20.54c3.905-3.904,10.237-3.904,14.143,0c3.905,3.905,3.905,10.237,0,14.143l-20.54,20.54      C33.999,338.482,31.44,339.459,28.88,339.459z")?;

    g2a.append_child(&path2a)?;
    g2.append_child(&g2a)?;

    let g2b = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2b = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2b.set_attribute("style", "fill:#5E2A41;")?;
    path2b.set_attribute("d", "M10,416.439c-2.56,0-5.119-0.977-7.072-2.93c-3.905-3.905-3.904-10.237,0.001-14.142l68.47-68.46      c3.905-3.904,10.237-3.904,14.142,0.001c3.905,3.905,3.904,10.237-0.002,14.142l-68.47,68.46      C15.118,415.463,12.559,416.439,10,416.439z")?;

    g2b.append_child(&path2b)?;
    g2.append_child(&g2b)?;

    let g2c = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2c = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2c.set_attribute("style", "fill:#5E2A41;")?;
    path2c.set_attribute("d", "M73.29,411.259c-2.56,0-5.118-0.977-7.071-2.929c-3.905-3.905-3.905-10.237,0-14.143      l34.23-34.229c3.905-3.904,10.237-3.903,14.142,0c3.905,3.905,3.905,10.237,0,14.143l-34.23,34.229      C78.409,410.282,75.849,411.259,73.29,411.259z")?;

    g2c.append_child(&path2c)?;
    g2.append_child(&g2c)?;

    let g2d = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2d = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2d.set_attribute("style", "fill:#5E2A41;")?;
    path2d.set_attribute("d", "M208.52,336.239c-2.56,0-5.118-0.977-7.071-2.929L83.139,215c-3.905-3.905-3.905-10.237,0-14.143      c3.905-3.904,10.237-3.904,14.143,0l118.31,118.311c3.905,3.905,3.905,10.237,0,14.143      C213.639,335.263,211.079,336.239,208.52,336.239z")?;

    g2d.append_child(&path2d)?;
    g2.append_child(&g2d)?;

    let g2e = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2e = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2e.set_attribute("style", "fill:#5E2A41;")?;
    path2e.set_attribute("d", "M259.58,218.534c-16.474,0-31.959-6.416-43.604-18.066c-11.646-11.641-18.062-27.126-18.062-43.6      c0-16.474,6.416-31.959,18.065-43.604c11.641-11.646,27.126-18.062,43.6-18.062s31.959,6.416,43.604,18.065      c11.645,11.641,18.061,27.126,18.061,43.6c0,16.472-6.415,31.956-18.061,43.6l0,0c-0.001,0.002-0.001,0.001-0.004,0.004      C291.536,212.119,276.052,218.534,259.58,218.534z M259.58,115.204c-11.13,0-21.592,4.334-29.457,12.204      c-7.874,7.869-12.208,18.331-12.208,29.461s4.334,21.592,12.204,29.457c7.869,7.874,18.331,12.208,29.461,12.208      c11.13,0,21.592-4.334,29.457-12.204c0.002-0.001,0.003-0.002,0.004-0.004c7.87-7.865,12.204-18.327,12.204-29.457      s-4.334-21.592-12.204-29.457C281.172,119.538,270.71,115.204,259.58,115.204z")?;

    g2e.append_child(&path2e)?;
    g2.append_child(&g2e)?;

    let g2f = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2f = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2f.set_attribute("style", "fill:#5E2A41;")?;
    path2f.set_attribute("d", "M89.291,296.31c-1.81,0-3.642-0.49-5.29-1.521c-4.684-2.926-6.108-9.096-3.182-13.779l30.49-48.8      c2.927-4.684,9.097-6.11,13.78-3.182c4.684,2.926,6.108,9.096,3.182,13.779l-30.49,48.8      C95.884,294.643,92.625,296.31,89.291,296.31z")?;

    g2f.append_child(&path2f)?;
    g2.append_child(&g2f)?;

    let g2g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2g.set_attribute("style", "fill:#5E2A41;")?;
    path2g.set_attribute("d", "M109.72,316.739c-2.559,0-5.118-0.977-7.071-2.929c-3.905-3.905-3.906-10.237-0.001-14.143      l39.65-39.65c3.905-3.904,10.237-3.904,14.142,0c3.905,3.905,3.906,10.237,0.001,14.142l-39.65,39.65      C114.839,315.763,112.279,316.739,109.72,316.739z")?;

    g2g.append_child(&path2g)?;
    g2.append_child(&g2g)?;

    let g2h = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2h = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2h.set_attribute("style", "fill:#5E2A41;")?;
    path2h.set_attribute("d", "M130.16,337.16c-3.334,0-6.593-1.666-8.49-4.702c-2.926-4.684-1.501-10.854,3.182-13.779      l48.8-30.49c4.683-2.929,10.853-1.503,13.78,3.182c2.926,4.684,1.501,10.853-3.182,13.779l-48.8,30.49      C133.801,336.67,131.97,337.16,130.16,337.16z")?;

    g2h.append_child(&path2h)?;
    g2.append_child(&g2h)?;

    let g2i = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2i = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2i.set_attribute("style", "fill:#5E2A41;")?;
    path2i.set_attribute("d", "M177.356,116.509c-2.307,0-4.625-0.794-6.512-2.415c-4.189-3.599-4.668-9.912-1.069-14.102      l33.71-39.24c3.598-4.188,9.911-4.668,14.102-1.068c4.189,3.599,4.668,9.912,1.068,14.101l-33.71,39.24      C182.968,115.327,180.17,116.509,177.356,116.509z")?;

    g2i.append_child(&path2i)?;
    g2.append_child(&g2i)?;

    let g2j = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2j = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2j.set_attribute("style", "fill:#5E2A41;")?;
    path2j.set_attribute("d", "M158.265,352.787c-10.448,0-20.723-4.085-28.34-11.712l-6.582-6.582      c-0.093-0.086-0.184-0.173-0.273-0.263l-47.694-47.695c-10.992-11.006-14.623-27.531-9.259-42.109l14.71-39.952      c0.413-1.12,1.022-2.157,1.799-3.061l87.14-101.42c3.601-4.188,9.913-4.667,14.102-1.068c4.189,3.6,4.667,9.913,1.068,14.102      L98.971,213.077l-14.086,38.257c-2.682,7.289-0.864,15.556,4.632,21.059l47.432,47.433c0.092,0.086,0.184,0.173,0.273,0.263      l6.85,6.85c5.497,5.504,13.756,7.318,21.048,4.63l38.252-14.089l139.302-119.675c4.191-3.6,10.504-3.119,14.102,1.068      c3.6,4.189,3.121,10.503-1.068,14.102L215.036,333.824c-0.904,0.777-1.94,1.387-3.059,1.799l-39.941,14.71      C167.557,351.985,162.893,352.787,158.265,352.787z")?;

    g2j.append_child(&path2j)?;
    g2.append_child(&g2j)?;

    let g2k = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2k = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2k.set_attribute("style", "fill:#5E2A41;")?;
    path2k.set_attribute("d", "M349.19,215.389c-2.559,0-5.118-0.977-7.071-2.929c-3.905-3.905-3.905-10.237,0-14.143      c19.885-19.884,34.642-43.315,43.863-69.644c11.736-33.512,13.626-69.25,5.536-103.733c-34.48-8.089-70.221-6.199-103.733,5.536      c-26.329,9.221-49.761,23.979-69.645,43.863c-3.905,3.904-10.236,3.905-14.143,0c-3.905-3.905-3.905-10.237,0-14.143      c22.025-22.024,47.991-38.375,77.176-48.596C320.331-2.111,362.231-3.69,402.344,7.039c3.454,0.924,6.152,3.622,7.076,7.076      c10.728,40.114,9.151,82.014-4.563,121.17c-10.221,29.185-26.571,55.15-48.596,77.175      C354.309,214.412,351.75,215.389,349.19,215.389z")?;

    g2k.append_child(&path2k)?;
    g2.append_child(&g2k)?;

    let g2l = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2l = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2l.set_attribute("style", "fill:#5E2A41;")?;
    path2l.set_attribute("d", "M395.41,141.98c-2.56,0-5.118-0.977-7.071-2.929L277.409,28.12      c-3.905-3.905-3.905-10.237,0-14.143c3.908-3.905,10.238-3.903,14.143,0l110.93,110.931c3.905,3.905,3.905,10.237,0,14.143      C400.528,141.003,397.969,141.98,395.41,141.98z")?;

    g2l.append_child(&path2l)?;
    g2.append_child(&g2l)?;

    let g2m = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2m = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2m.set_attribute("style", "fill:#5E2A41;")?;
    path2m.set_attribute("d", "M90.22,217.929c-0.832,0-1.67-0.104-2.477-0.309l-66.33-16.88      c-3.037-0.773-5.537-2.926-6.751-5.814c-1.215-2.889-1.005-6.181,0.566-8.892c7.778-13.418,17.355-25.86,28.467-36.982      c35.281-35.281,84.119-54.445,133.988-52.537c5.369,0.176,9.671,4.583,9.671,9.994c0,5.522-4.472,10-9.995,10h-0.01      c-0.127,0-0.254-0.002-0.381-0.007c-44.338-1.699-87.765,15.325-119.127,46.688c-6.684,6.689-12.742,13.914-18.101,21.576      l52.73,13.419c4.435,1.024,7.745,4.998,7.745,9.743C100.215,213.451,95.743,217.929,90.22,217.929z")?;

    g2m.append_child(&path2m)?;
    g2.append_child(&g2m)?;

    let g2n = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

    let path2n = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    path2n.set_attribute("style", "fill:#5E2A41;")?;
    path2n.set_attribute("d", "M225.41,402.579c-1.315,0-2.633-0.259-3.876-0.782c-2.89-1.215-5.042-3.714-5.815-6.75      l-16.891-66.34c-1.363-5.353,1.872-10.796,7.224-12.158c5.349-1.366,10.795,1.871,12.158,7.223l13.48,52.948      c7.663-5.359,14.889-11.419,21.581-18.104c31.36-31.36,48.378-74.785,46.684-119.136c-0.21-5.519,4.093-10.163,9.611-10.374      c5.509-0.233,10.164,4.093,10.375,9.611c1.903,49.897-17.243,98.755-52.532,134.044c-11.124,11.113-23.567,20.691-36.986,28.47      C228.881,402.126,227.148,402.579,225.41,402.579z")?;

    g2n.append_child(&path2n)?;
    g2.append_child(&g2n)?;
    g.append_child(&g2)?;

    base_g.append_child(&g2)?;

    svg.append_child(&base_g).unwrap();
    div.append_child(&svg).unwrap();
    figure.append_child(&div).unwrap();
    body.append_child(&figure).unwrap();

    Ok(())
}


#[wasm_bindgen]
pub fn start() -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    return Ok(context);
}

#[wasm_bindgen]
pub fn draw_face(ctx: web_sys::CanvasRenderingContext2d) {
    // Draw the outer circle.
    ctx
    .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    .unwrap();

    // Draw the mouth.
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    ctx.move_to(65.0, 65.0);
    ctx
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    ctx.move_to(95.0, 65.0);
    ctx
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    ctx.stroke();
}


#[wasm_bindgen]
pub fn draw_line(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64) {
    ctx.line_to(x, y);

    ctx.stroke();
}

#[wasm_bindgen]
pub fn draw_rect(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
    ctx.rect(x, y, w, h);

    ctx.stroke();
}

#[wasm_bindgen]
pub fn draw_text(ctx: web_sys::CanvasRenderingContext2d, text: &str, x: f64, y: f64, w: f64) {
    ctx.stroke_text_with_max_width(text, x, y, w);
}

#[wasm_bindgen]
pub fn set_color(ctx: web_sys::CanvasRenderingContext2d, text: &str) {
    ctx.set_shadow_color(text);
}