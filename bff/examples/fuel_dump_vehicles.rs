#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
bff = { path = ".." }
clap = { version = "4.5.60", features = ["derive"] }
---

#![allow(duplicate_features)]
#![feature(frontmatter)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use bff::tsc::{AsoboExecutor, FileSystemScriptLoader};
use clap::Parser;

#[derive(Parser)]
#[command(about = "List FUEL vehicles and their English translation strings")]
struct Arguments {
    /// FUEL root directory, containing `user.tsc` and `trtext/tt01.pc`.
    root: PathBuf,
}

struct Vehicle {
    mesh_name: String,
    tt_id: u32,
}

#[derive(Default)]
struct DumpData {
    translations: BTreeMap<u32, String>,
    vehicles: Vec<Vehicle>,
}

fn main() {
    let arguments = Arguments::parse();
    let mut executor = AsoboExecutor::with_user_data(
        FileSystemScriptLoader::new(arguments.root),
        DumpData::default(),
    );
    executor.set_variable("_PC");
    executor.set_variable("_MASTER");
    executor.set_variable("_BIGFILE");
    executor.on_command("TransText", |executor, command| {
        let id = command.arguments[0].string.parse().unwrap();
        let text = command.arguments[1].string.clone();
        executor.user_data_mut().translations.insert(id, text);
        Ok(())
    });
    executor.on_command("AddVehicleInfo", |executor, command| {
        executor.user_data_mut().vehicles.push(Vehicle {
            mesh_name: command.arguments[0].string.clone(),
            tt_id: command.arguments[5].string.parse().unwrap(),
        });
        Ok(())
    });
    executor.execute_file("trtext/tt01.pc").unwrap();
    executor.execute_file("user.tsc").unwrap();

    let data = executor.into_user_data();
    for vehicle in data.vehicles {
        let translation = data.translations.get(&vehicle.tt_id).unwrap().as_str();
        println!("{}\t{}\t{}", vehicle.mesh_name, vehicle.tt_id, translation);
    }
}
