// use std::env;
// use std::fs;
// use std::path::Path;

use protobuf_codegen_pure::Customize;

fn main() {

    protoc_rust::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir("src/protos")
        .inputs(&["protos/def.proto", "protos/test.proto"])
        .include("protos")
        .run()
        .expect("protoc");
}