#![feature(path_try_exists)]

use std::env::current_dir;
use std::fs::{copy, create_dir_all, read_dir, try_exists};
use std::process::Command;

enum BuildSystem {
    Cargo,
    Makefile,
}

fn main() {
	println!("cargo:rerun-if-changed=example_bots/");

	let original_dir = current_dir().unwrap();
	let mut bot_dir = original_dir.clone();
	bot_dir.push("example_bots/");

	for bot_entry in read_dir(bot_dir).unwrap() {
		let bot_entry = bot_entry.unwrap();

		// Skip any entries that are not directories
		if !bot_entry.metadata().unwrap().is_dir() {
			continue;

		}

        let bot_dir = bot_entry.path();
       
        println!("{:#?}", bot_dir);

        let build_system = {
            let cargo_path = {
                let mut dir = bot_dir.clone();
                dir.push("Cargo.toml");
                dir

            };

            let makefile_path = {
                let mut dir = bot_dir.clone();
                dir.push("Makefile");
                dir

            };

            if Some(true) == try_exists(&cargo_path).ok() {
                BuildSystem::Cargo

            } else if Some(true) == try_exists(&makefile_path).ok() {
                BuildSystem::Makefile

            } else {
                panic!("Unknown build system");

            }

        };


		let wasm_file_name = bot_dir.file_name().unwrap().to_str().unwrap();

        match build_system {
		    BuildSystem::Cargo => assert!(
                Command::new("cargo")
                .env_remove("CARGO_CFG_TARGET_ARCH")
		        .env_remove("CARGO_CFG_TARGET_ENDIAN")
		        .env_remove("CARGO_CFG_TARGET_FAMILY")
		        .env_remove("CARGO_CFG_TARGET_ENV")
		        .env_remove("CARGO_CFG_TARGET_FEATURE")
		        .env_remove("CARGO_CFG_TARGET_OS")
		        .env_remove("CARGO_ENCODED_RUSTFLAGS")
		        .current_dir(bot_dir.clone())
		        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
				.status()
				.unwrap()
                .success()
            ),
            BuildSystem::Makefile => assert!(
             Command::new("make")
                .current_dir(bot_dir.clone())
                .status()
                .unwrap()
                .success()
            ),
        };

			let mut dst_dir = bot_dir.clone();
            let mut bot_alg_dir = dst_dir.clone();
            bot_alg_dir.push("../../../../bot_algs/");
			
            create_dir_all(bot_alg_dir);
            dst_dir.push(&format!("../../../../bot_algs/{wasm_file_name}.wasm"));

			let mut wasm_file = bot_dir.clone();
			
            match build_system {
                BuildSystem::Cargo => wasm_file.push(&format!("./target/wasm32-unknown-unknown/release/{wasm_file_name}.wasm")),
                BuildSystem::Makefile => wasm_file.push(&format!("./{wasm_file_name}.wasm")),
            };

			if copy(&wasm_file, &dst_dir).is_err() {
                panic!("WASM dir: {wasm_file:#?}\ndst: {dst_dir:#?}");

            }

	}

}
