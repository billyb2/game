use std::env::{current_dir, set_current_dir};
use std::fs::copy;
use std::process::Command;

fn main() {
	println!("cargo:rerun-if-changed=example_bots/aggro_bot/src");

	let original_dir = current_dir().unwrap();
	let mut aggro_bot_dir = original_dir.clone();
	aggro_bot_dir.push("example_bots/aggro_bot");

	assert!(Command::new("cargo")
        .env_remove("CARGO_CFG_TARGET_ARCH")
        .env_remove("CARGO_CFG_TARGET_ENDIAN")
        .env_remove("CARGO_CFG_TARGET_FAMILY")
        .env_remove("CARGO_CFG_TARGET_ENV")
        .env_remove("CARGO_CFG_TARGET_FEATURE")
        .env_remove("CARGO_CFG_TARGET_OS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(aggro_bot_dir.clone())
        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
		.status()
		.unwrap()
		.success());

	let mut dst_dir = aggro_bot_dir.clone();
	dst_dir.push("../aggro_bot.wasm");

	aggro_bot_dir.push("./target/wasm32-unknown-unknown/release/aggro_bot.wasm");
	copy(aggro_bot_dir, dst_dir).unwrap();

}
