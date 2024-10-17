use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    build_gf_complete();
    build_jerasure();
    bindgen_jerasure();
}

fn build_gf_complete() {
    const _MIN_VERSION: &str = "2.0";
    const LIB_NAME: &str = "gf-complete";
    const MODULE_DIR: &str = "./vendor/gf-complete";

    // Submodule directory containing upstream source files (readonly)
    let module_dir = std::fs::canonicalize(MODULE_DIR).expect("gf-complete directory not found");

    // Copy source files to writable directory in `OUT_DIR`
    let out_src_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap())
        .join("src")
        .join(LIB_NAME);
    create_dir_all(&out_src_dir)
        .unwrap_or_else(|_| panic!("Failed to create {}", out_src_dir.display()));
    cp_r(module_dir, out_src_dir.clone());

    // Run `./autogen.sh`
    Command::new("sh")
        .current_dir(out_src_dir.clone())
        .arg("autogen.sh")
        .status()
        .unwrap();

    // Build using autotools
    let install_root_dir = autotools::build(out_src_dir);

    // link the library
    println!(
        "cargo:rustc-link-search=native={}",
        install_root_dir.display()
    );
    println!("cargo:rustc-link-lib=static={}", LIB_NAME);
}

fn build_jerasure() {
    const _MIN_VERSION: &str = "2.0";
    const LIB_NAME: &str = "jerasure";
    const MODULE_DIR: &str = "./vendor/jerasure";

    // Submodule directory containing upstream source files (readonly)
    let module_dir = std::fs::canonicalize(MODULE_DIR).expect("jerasure directory not found");

    // Copy source files to writable directory in `OUT_DIR`
    let out_src_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap())
        .join("src")
        .join(LIB_NAME);
    create_dir_all(&out_src_dir)
        .unwrap_or_else(|_| panic!("Failed to create {}", out_src_dir.display()));
    cp_r(module_dir, out_src_dir.clone());

    // Run `autorecofig`
    Command::new("autoreconf")
        .current_dir(out_src_dir.clone())
        .args(&["--force", "--install"])
        .status()
        .unwrap();

    // Build using autotools
    let _install_root_dir = autotools::Config::new(out_src_dir.clone())
        // .reconf("--force --install")
        .build();
    println!("cargo:rustc-link-lib=static={}", LIB_NAME);
}

fn bindgen() {
    // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // let include_dir = out_path.join("include");
    // fn find_headers(dir: &Path, headers: &mut Vec<PathBuf>) {
    //     for entry in std::fs::read_dir(dir).unwrap() {
    //         let entry = entry.unwrap();
    //         let path = entry.path();
    //         if path.is_dir() {
    //             find_headers(&path, headers);
    //         } else {
    //             if path.extension().map(|e| e == "h").is_some() {
    //                 headers.push(path);
    //             }
    //         }
    //     }
    // }
    // let mut headers = Vec::new();
    // find_headers(&include_dir, &mut headers);
    // let wrapper_path = out_path.join("wrapper.h");
    // let mut wrapper = std::fs::File::options()
    //     .create(true)
    //     .truncate(true)
    //     .write(true)
    //     .read(true)
    //     .open(&wrapper_path)
    //     .unwrap();
    // headers.iter().for_each(|path| {
    //     writeln!(wrapper, "#include \"{}\"", path.display()).unwrap();
    // });
    // bindgen::Builder::default()
    //     .header(wrapper_path.to_str().unwrap())
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}

fn bindgen_jerasure() {
    const INCLUDE_DIR: &str = "./vendor/jerasure/include";
    const OUT_DIR: &str = "./src/bind_sys.rs";
    bindgen::Builder::default()
        .header(
            PathBuf::from(INCLUDE_DIR)
                .join("jerasure.h")
                .to_str()
                .unwrap(),
        )
        .header(
            PathBuf::from(INCLUDE_DIR)
                .join("galois.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_item("jerasure_.*")
        .allowlist_item("galois_.*")
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(OUT_DIR)
        .expect("Couldn't write bindings!");
}

fn cp_r(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    for e in from.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let from = e.path();
        let to = to.as_ref().join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            std::fs::create_dir_all(&to).unwrap();
            cp_r(&from, &to);
        } else {
            println!("{} => {}", from.display(), to.display());
            std::fs::copy(&from, &to).unwrap();
        }
    }
}
