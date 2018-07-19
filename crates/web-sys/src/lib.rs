#![feature(wasm_import_module, use_extern_macros, extern_prelude)]
extern crate wasm_bindgen;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
