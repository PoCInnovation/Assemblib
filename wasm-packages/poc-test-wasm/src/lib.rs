// use std::convert::TryInto;
use std::f64;
use std::{thread, time};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement};

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
pub fn start(id: &str) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
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

    return Ok(context);
}

#[wasm_bindgen]
pub fn sleep(ms: i32) {
    let closure = Closure::wrap(Box::new(move || {
    }) as Box<dyn FnMut()>);
    let cb = closure.as_ref().unchecked_ref();
    let _promise = web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb, ms);
    closure.forget();
}

#[wasm_bindgen]
pub fn draw_line(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64) {
    ctx.line_to(x, y);
}

#[wasm_bindgen]
pub fn draw_rect(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
    ctx.rect(x, y, w, h);
}

#[wasm_bindgen]
pub fn sleep_milliseconds(duration: u64) {
    thread::sleep(time::Duration::from_secs(duration));
}

#[wasm_bindgen]
pub fn draw_text(ctx: web_sys::CanvasRenderingContext2d, text: &str, x: f64, y: f64, w: f64) -> Result<(), JsValue> {
    return ctx.stroke_text_with_max_width(text, x, y, w);
}

#[wasm_bindgen]
pub fn set_color(ctx: web_sys::CanvasRenderingContext2d, color: &str) {
    ctx.set_fill_style(&color.into());
}

#[wasm_bindgen]
pub fn clear_canva(ctx: web_sys::CanvasRenderingContext2d, w: f64, h: f64) {
    ctx.move_to(0.0, 0.0);
    ctx.clear_rect(0.0, 0.0, w, h);
}

#[wasm_bindgen]
pub fn clear_rect(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
    ctx.clear_rect(x, y, w, h);
}

#[wasm_bindgen]
pub fn draw_circle(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, s: f64) -> Result<(), JsValue> {
    let arc_res = ctx.arc(x, y, s, 0.0, 2.0 * std::f64::consts::PI);

    return arc_res;
}

#[wasm_bindgen]
pub fn stroke(ctx: web_sys::CanvasRenderingContext2d) {
    ctx.stroke();
}

#[wasm_bindgen]
pub fn fill(ctx: web_sys::CanvasRenderingContext2d) {
    ctx.fill();
}

#[wasm_bindgen]
pub fn draw_arc(ctx: web_sys::CanvasRenderingContext2d, x: f64, y: f64, s: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue> {
    let arc_res = ctx.arc(x, y, s, start_angle, end_angle);

    return arc_res;
}

#[wasm_bindgen]
pub fn start_drawing(ctx: web_sys::CanvasRenderingContext2d) {
    ctx.begin_path();
}

#[wasm_bindgen]
pub fn stop_drawing(ctx: web_sys::CanvasRenderingContext2d) {
    ctx.close_path();
}

// ! refacto code to add classe
// TODO add color
// TODO add animations