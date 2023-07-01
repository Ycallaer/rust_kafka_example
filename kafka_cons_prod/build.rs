extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/identity.proto"],
                                &["src/"]).unwrap();
}