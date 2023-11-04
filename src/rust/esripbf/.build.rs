 fn main() {
    let mut conf = prost_build::Config::new();
    conf.out_dir("src/");
    conf.compile_protos(
        &["src/FeatureCollection.proto"],
        &["src/"]
    ).unwrap();
}

