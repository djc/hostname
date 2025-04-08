#![cfg(windows)]

use std::fs;
use std::process::Command;

use windows_bindgen::bindgen;

#[test]
fn gen_bindings() {
    bindgen([
        "--no-deps",
        "--etc",
        "--out src/windows/bindings.rs",
        "--flat",
        "--sys",
        "--no-comment",
        "--filter",
        "ComputerNamePhysicalDnsHostname",
        "GetComputerNameExW",
        "SetComputerNameExW",
    ]);
}
