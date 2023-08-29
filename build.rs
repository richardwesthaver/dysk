fn main() -> Result<(), cbindgen::Error> {
  if let Ok(b) = cbindgen::generate(std::env::var("CARGO_MANIFEST_DIR").unwrap()) {
    b.write_to_file("dysk.h"); Ok(())}
  else { panic!("failed to generate dysk.h from cbindgen.toml") } }
