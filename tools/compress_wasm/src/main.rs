fn main() {
	use std::fs::{read, read_to_string, write};
	use std::io::Cursor;
	use std::env::args;

	use brotli::BrotliCompress;
	use brotli::enc::BrotliEncoderParams;

	let (wasm_path, compressed_wasm_path, js_path, wasm_decompress_js_path) = {
		let working_dir =  std::env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap_or(String::from("./"));

		let mut wasm_path = working_dir.clone();
		wasm_path.push_str("/target/wasm-simd_bg.wasm");

		let mut compressed_wasm_path = working_dir;
		compressed_wasm_path.push_str("/target/wasm-simd_bg.wasm.br");

		let mut js_path = std::env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap();
		js_path.push_str("/target/wasm-simd.js");

		let mut wasm_decompress_js_path = std::env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap();
		wasm_decompress_js_path.push_str("/wasm_decompress.js");

		(wasm_path, compressed_wasm_path, js_path, wasm_decompress_js_path)

	};

	let wasm_decompress_js = read_to_string(wasm_decompress_js_path).unwrap();
	let mut js = read_to_string(js_path.clone()).unwrap().replace("return await WebAssembly.instantiateStreaming(module, imports);", "const { decompress } = wasm_bindgen;await wasm_bindgen('./wasm_decompress_bg.wasm');let bytes = decompress(new Uint8Array(await module.arrayBuffer())).buffer;return await WebAssembly.instantiate(bytes, imports);");

	js.insert_str(0, &wasm_decompress_js);

	let wasm_bytes = read(wasm_path).unwrap();
	let len = wasm_bytes.len();
	let mut wasm_bytes = Cursor::new(wasm_bytes);


	let mut compressed_bytes = Vec::with_capacity(3000000);

	println!("Original WASM file length: {:.1}MB", len as f32 / 1000000.0);
	println!("Compressing WASM file...");

    let mut args = args();
    // First arg is just the bin name
    args.next();

	let mut params = BrotliEncoderParams::default();
	params.quality = match args.next() {
		Some(quality) => quality.parse().unwrap(),
		None => {
			println!("No compression level provided; defaulting to 2");
			2
		},

	};

	BrotliCompress(&mut wasm_bytes, &mut compressed_bytes, &params).unwrap();

	println!("New compressed WASM file length: {:.1}MB\nThe compressed file is {:.1}% the size of the uncompressed one", compressed_bytes.len() as f32 / 1000000.0, (compressed_bytes.len() as f32 / len as f32) * 100.0 );

	println!("Writing new wasm file...");
	write(compressed_wasm_path, compressed_bytes).unwrap();
    write(js_path, js.as_bytes()).unwrap();

}
