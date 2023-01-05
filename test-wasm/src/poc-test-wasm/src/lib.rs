// use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};



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

// #[wasm_bindgen]
// pub fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn compute_fibonacci(n: i32) -> i32 {
//     match n {
//         0 | 1 => return n.try_into().unwrap(),
//         _ => return compute_fibonacci(n - 1) + compute_fibonacci(n - 2),
//     }
// }

// #[wasm_bindgen]
// pub fn fibonacci(value: i32) -> Vec<i32> {
//     let mut numbers: Vec<i32> = Vec::new();
//     for i in 0..value + 1 {
//         numbers.push(compute_fibonacci(i));
//     }
//     numbers
// }

// #[wasm_bindgen]
// pub fn display_red_circle() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
//     svg.set_attribute("width", "100")?;
//     svg.set_attribute("hight", "100")?;
//     svg.set_attribute("viewBox", "0 0 100 100")?;

//     let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
//     circle.set_attribute("cx", "50")?;
//     circle.set_attribute("cy", "50")?;
//     circle.set_attribute("r", "20")?;
//     circle.set_attribute("stroke", "black")?;
//     circle.set_attribute("fill", "red")?;
//     svg.append_child(&circle)?;
//     body.append_child(&svg)?;

//     Ok(())
// }

// #[wasm_bindgen]
// pub fn display_blue_circle() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
//     svg.set_attribute("width", "100")?;
//     svg.set_attribute("hight", "100")?;
//     svg.set_attribute("viewBox", "0 0 100 100")?;

//     let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
//     circle.set_attribute("cx", "50")?;
//     circle.set_attribute("cy", "50")?;
//     circle.set_attribute("r", "20")?;
//     circle.set_attribute("fill", "blue")?;
//     svg.append_child(&circle)?;
//     body.append_child(&svg)?;

//     Ok(())
// }

// #[wasm_bindgen]
// pub fn display_blue_square() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
//     svg.set_attribute("width", "100")?;
//     svg.set_attribute("hight", "100")?;
//     svg.set_attribute("viewBox", "0 0 100 100")?;

//     let square = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
//     square.set_attribute("width", "100")?;
//     square.set_attribute("height", "100")?;
//     square.set_attribute("fill", "blue")?;
//     svg.append_child(&square)?;
//     body.append_child(&svg)?;

//     Ok(())
// }

// #[wasm_bindgen]
// pub fn display_calendar() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
//     svg.set_attribute("width", "64")?;
//     svg.set_attribute("height", "64")?;
//     svg.set_attribute("viewBox", "0 0 24 24")?;

//     let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
//     path.set_attribute(
//         "d",
//         "M 4 4
//         h 16
//         a 2 2 0 0 1 2 2
//         v 14
//         a 2 2 0 0 1 -2 2
//         h -16
//         a 2 2 0 0 1 -2 -2
//         v -14
//         a 2 2 0 0 1 2 -2",
//     )?;

//     let line_1 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
//     line_1.set_attribute("x1","2")?;
//     line_1.set_attribute("y1","10")?;
//     line_1.set_attribute("x2","22")?;
//     line_1.set_attribute("y2","10")?;

//     let line_2 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
//     line_2.set_attribute("x1","7")?;
//     line_2.set_attribute("y1","2")?;
//     line_2.set_attribute("x2","7")?;
//     line_2.set_attribute("y2","6")?;

//     let line_3 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
//     line_3.set_attribute("x1","17")?;
//     line_3.set_attribute("y1","2")?;
//     line_3.set_attribute("x2","17")?;
//     line_3.set_attribute("y2","6")?;

//     svg.append_child(&path)?;
//     svg.append_child(&line_1)?;
//     svg.append_child(&line_2)?;
//     svg.append_child(&line_3)?;
//     body.append_child(&svg)?;

//     Ok(())
// }

// #[wasm_bindgen]
// #[derive(Debug)]
// pub struct DocumentStruct {
//     document: Document,
//     body: HtmlElement,
// }

// #[wasm_bindgen]
// pub struct Svg {
//     document_info: DocumentStruct,
// }

// #[wasm_bindgen]
// impl Svg {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Svg {
//         let window = web_sys::window().expect("no global `window` exists");
//         let document = window.document().expect("should have a document on window");
//         let body = document.body().expect("document should have a body");
//         let document_info = DocumentStruct {document, body};
//         return Svg {document_info};
//     }
// }

#[wasm_bindgen]
pub struct DrawShapes {
    document: Document,
    body: HtmlElement,
}

#[wasm_bindgen]
impl DrawShapes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DrawShapes {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        return DrawShapes {document, body};
    }

    pub fn circle(&self) -> Result<(), JsValue> {
        let svg = self.document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
        svg.set_attribute("width", "100")?;
        svg.set_attribute("hight", "100")?;
        svg.set_attribute("viewBox", "0 0 100 100")?;

        let circle = self.document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        circle.set_attribute("cx", "50")?;
        circle.set_attribute("cy", "50")?;
        circle.set_attribute("r", "20")?;
        circle.set_attribute("fill", "blue")?;
        svg.append_child(&circle)?;
        self.body.append_child(&svg)?;

        Ok(())
    }
}
