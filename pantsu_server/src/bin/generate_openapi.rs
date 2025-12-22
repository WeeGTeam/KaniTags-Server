use pantsu_lib::config::ServerConfig;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() -> Result<(), std::io::Error> {
    let (_, api) = pantsu_lib::routes::get_router(&ServerConfig::default()).split_for_parts();
    let cargo_toml_dir = PathBuf::from_str(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();
    let target_dir = cargo_toml_dir.join("..").join("target").canonicalize()?;
    let file_path = target_dir.join("openapi.json");
    println!("writing openapi spec to {:?}", file_path);
    fs::write(file_path, api.to_pretty_json()?)
}
