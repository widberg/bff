#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod bigfile;
mod cps;
mod mqfel_settings;
mod path_helpers;
