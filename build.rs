use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("sensor_api_c").canonicalize().unwrap();

    if !std::process::Command::new("bash")
        .arg("compile.sh")
        .output()
        .expect("could not spawn bash compile script")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("compilation exited with non-zero");
    }

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        libdir_path.join("build").to_str().unwrap()
    );

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=bme280");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("wrapper.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");
    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
