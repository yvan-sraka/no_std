#![no_std]

use hs_bindgen::*;

#[hs_bindgen(hello :: CString -> IO ())]
fn greetings(_name: *const i8) {}

fn main() {}
