 fn main() {
    println!("cargo:rerun-if-changed=src/FeatureCollection.proto");
    let mut conf = prost_build::Config::new();
    conf.out_dir("src/");
    conf.compile_protos(
        &["src/FeatureCollection.proto"],
        &["src/"]
    ).unwrap();
}

