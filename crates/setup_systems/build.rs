use std::env::{current_dir, set_current_dir};
use std::fs::copy;
use std::process::Command;

fn main() {
	/*println!("cargo:rerun-if-changed=../bots/example/aggro_bot");

	let original_dir = current_dir().unwrap();
	let mut aggro_bot_dir = original_dir.clone();
	aggro_bot_dir.push("../bots/example_bots/aggro_bot");

	set_current_dir(aggro_bot_dir.clone()).unwrap();

	assert!(Command::new("cargo")
		.args(["build", "--target", "wasm32-unknown-unknown", "--release"])
		.status()
		.unwrap()
		.success());

	let mut dst_dir = aggro_bot_dir.clone();
	dst_dir.push("../");

	aggro_bot_dir.push("./target/wasm32-unknown-unknown/release/aggro_bot.wasm");
	println!("{:?}\n{:?}", aggro_bot_dir, dst_dir);

	copy(aggro_bot_dir, dst_dir).unwrap();

	set_current_dir(original_dir).unwrap();*/

}