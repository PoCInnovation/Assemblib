mod utils;

use wasm_bindgen::prelude::*;
use std::convert::TryInto;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, poc-test-wasm!");
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn compute_fibonacci(n: usize) -> i64 {
    match n {
        0 | 1 => return n.try_into().unwrap(),
        _ => return compute_fibonacci(n-1) + compute_fibonacci(n-2),
    }
}

#[wasm_bindgen]
pub fn fibonacci(value: usize) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    for i in 0..value + 1 {
        numbers.push(compute_fibonacci(i));
    }
    numbers
}
