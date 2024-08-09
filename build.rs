use dbc_codegen::Config;
use std::path::PathBuf;

fn main() {
    let dbc_path = "dbc/demo.dbc";
    let dbc_file = std::fs::read(dbc_path).unwrap();

    // rerun if the dbc file is changed
    println!("cargo:rerun-if-changed={}", dbc_path);

    // rerun if this build script is changed
    println!("cargo:rerun-if-changed=build.rs");

    // get the cargo defined output directory
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut out_file = PathBuf::new();
    out_file.push(out_dir);
    out_file.push("messages.rs");

    // dbc-codegen configuration
    let config = Config::builder()
        .dbc_name("demo.dbc")
        .dbc_content(&dbc_file)
        .build();

    let mut out = std::io::BufWriter::new(std::fs::File::create(out_file).unwrap());
    dbc_codegen::codegen(config, &mut out).expect("dbc-codegen failed");
}
