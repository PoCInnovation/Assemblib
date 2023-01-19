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

fn _attributes_to_hashmap(attributes_str: &str) -> HashMap<String, &str> {
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
pub fn draw_rect(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, filled: bool, c: Option<String>) {
    let _color: String = c.unwrap_or(String::from("black"));

    ctx.rect(x, y, w, h);
    if filled {
        // ctx.fillStyle = color;
        ctx.fill();
    } else {
        // ctx.strokeStyle = color;
        ctx.stroke();
    }
}

// #[wasm_bindgen]
// pub fn draw_image(ctx: web_sys::CanvasRenderingContext2d, img_id: &str, x: f64, y: f64) -> Result<(), JsValue> {
//     let document = web_sys::window().unwrap().document().unwrap();
//     let image = document.get_element_by_id(img_id).unwrap();

//     ctx.draw_image_with_html_canvas_element(image as &HtmlCanvasElement, x, y).unwrap();

//     return Ok(());
// }

#[wasm_bindgen]
pub fn draw_text(ctx: web_sys::CanvasRenderingContext2d, text: &str, x: f64, y: f64, w: f64) -> Result<(), JsValue> {
    return ctx.stroke_text_with_max_width(text, x, y, w);
}

#[wasm_bindgen]
pub fn set_color(ctx: web_sys::CanvasRenderingContext2d, text: &str) {
    ctx.set_shadow_color(text);
}

#[wasm_bindgen]
pub fn clear_canva(ctx: web_sys::CanvasRenderingContext2d, w: f64, h: f64) {
    ctx.clear_rect(0.0, 0.0, w, h);
}

#[wasm_bindgen]
pub fn clear_rect(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
    ctx.clear_rect(x, y, w, h);
}

#[wasm_bindgen]
pub fn draw_circle(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, s: f64) -> Result<(), JsValue> {
    let arc_res = ctx.arc(x, y, s, 0.0, 2.0 * std::f64::consts::PI);

    ctx.stroke();

    return arc_res;
}

#[wasm_bindgen]
pub fn draw_arc(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, s: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue> {
    let arc_res = ctx.arc(x, y, s, start_angle, end_angle);

    ctx.stroke();

    return arc_res;
}