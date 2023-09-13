use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(num: i32) -> i32 {
    if num <= 1 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}

#[wasm_bindgen]
pub fn result(num: i32) {
    let fibonacci_result = fibonacci(num);
    alert(&format!("num: {}, fibonacci result {}", num, fibonacci_result))
}
