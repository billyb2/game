use std::fs::write;
use inline_spirv::include_spirv;

fn main() {
	println!("cargo:rerun-if-changed=assets/shaders/*.frag");

	// Compiles all the shaders
	let compiled_spv_vert: Vec<u8> = include_spirv!("assets/shaders/sprite.vert", vert, glsl, max_perf, auto_bind).iter().map(|int| int.to_ne_bytes()).flatten().collect();
	let compiled_spv_player_frag: Vec<u8> = include_spirv!("assets/shaders/player.frag", frag, glsl, max_perf).iter().map(|int| int.to_ne_bytes()).flatten().collect();
	let compiled_spv_lighting_frag: Vec<u8> = include_spirv!("assets/shaders/lighting.frag", frag, glsl, max_perf, auto_bind).iter().map(|int| int.to_ne_bytes()).flatten().collect();

	write("assets/shaders/sprite.vert.spv", &compiled_spv_vert).unwrap();
	write("assets/shaders/player.frag.spv", &compiled_spv_player_frag).unwrap();
	write("assets/shaders/lighting.frag.spv", &compiled_spv_lighting_frag).unwrap();

}