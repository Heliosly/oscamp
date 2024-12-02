#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("[WithColor]: Hello, Arceos!");
     println!("\x1b[33m[WithColor]: Hello, Arceos!\x1b[0m");
}
