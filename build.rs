use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from("src/proto/generated");
    
    // Create the proto output directories if they don't exist
    fs::create_dir_all(&out_dir).unwrap();
    fs::create_dir_all(&src_dir).unwrap();
    
    // Generate the Rust code from proto
    protobuf_codegen::Codegen::new()
        .pure()
        .includes(&["proto"])
        .input("proto/kymera_mappings.proto")
        .out_dir(&src_dir)
        .customize(protobuf_codegen::Customize::default()
            .generate_accessors(true)
            .generate_getter(true))
        .run_from_script();

    // Tell cargo to rerun this script if the proto files change
    println!("cargo:rerun-if-changed=proto/test.proto");
    println!("cargo:rerun-if-changed=proto/kymera_mappings.proto");
} 