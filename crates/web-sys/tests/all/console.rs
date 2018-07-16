use super::websys_project;

#[test]
fn console() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_console() {
                    assert!(web_sys::console::log());
                    assert!(web_sys::console::debug());
                    assert!(web_sys::console::assert());
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export async function test() {
                  wasm.test_console();
                }
            "#,
        )
        .test();
}
