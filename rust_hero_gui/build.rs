fn main() {
    use std::env;
    use std::path::PathBuf;
    println!("cargo:rerun-if-changed=src/ui.fld");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/ui.fld", out_path.join("ui.rs").to_str().unwrap())
        .expect("Failed to generate rust from fl file!");
}
