use std::path::PathBuf;
use wabble_data::parse::parse_data;

fn main() {
    println!("Parsing wabble data...");
    let resources_path = PathBuf::from("./data/resources");
    let data = parse_data(&resources_path).unwrap();
    println!("Encoding data...");
    let uncompressed = bincode::encode_to_vec(data, bincode::config::standard()).unwrap();
    println!("Data encoded: {} bytes", uncompressed.len());
    println!("Compressing data...");
    let compressed = zstd::encode_all(uncompressed.as_slice(), 22).unwrap();
    println!("Data compressed: {} bytes", compressed.len());

    let output_path = PathBuf::from("./data.bin");
    std::fs::write(output_path, compressed).unwrap();
    println!("Done!");
}
