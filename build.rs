fn main() {
     fn main() {
          let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
          println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("build").display());
     }
}
