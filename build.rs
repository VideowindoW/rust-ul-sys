use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ultralight_dir = out_dir.join("Ultralight");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper");

    if ultralight_dir.is_dir() {
        fs::remove_dir_all(&ultralight_dir)
            .expect("Could not remove already existing Ultralight repo");
    }

    let git_status = Command::new("git")
        .args(&["clone", "https://github.com/ultralight-ux/Ultralight"])
        .current_dir(&out_dir)
        .status()
        .expect("Git is needed to retrieve the ultralight C++ library!");

    assert!(git_status.success(), "Couldn't clone Ultralight library");

    let git_status = Command::new("git")
        .args(&[
            "reset",
            "--hard",
            "36726f76a13fd0c3416a3cb2b2b323a101c00f2a",
        ])
        .current_dir(&ultralight_dir)
        .status()
        .expect("Git is needed to retrieve the ultralight C++ library!");

    assert!(
        git_status.success(),
        "Could not reset git head to desired revision"
    );

    let dst = cmake::build(ultralight_dir.join("packager"));
    let lib_bin_dir = dst.join("bin");

    if cfg!(feature = "only-ul-deps") {
        let allowed_files = [
            "Ultralight",
            "UltralightCore",
            "WebCore",
            "AppCore",
            "gstreamer-full-1.0",
        ];
        for entry in fs::read_dir(&lib_bin_dir).unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();

                let mut allowed = false;
                for allowed_file in &allowed_files {
                    let filename = path.file_name().unwrap().to_str();
                    if let Some(filename) = filename {
                        if filename.contains(allowed_file) {
                            allowed = true;
                            break;
                        }
                    }
                }

                if !allowed
                    && entry
                        .file_type()
                        .map(|f| f.is_file() || f.is_symlink())
                        .unwrap_or(false)
                {
                    fs::remove_file(entry.path()).unwrap();
                }
            }
        }
    }

    println!("cargo:rustc-link-search=native={}", lib_bin_dir.display());

    println!("cargo:rustc-link-lib=dylib=Ultralight");
    println!("cargo:rustc-link-lib=dylib=WebCore");
    println!("cargo:rustc-link-lib=dylib=AppCore");

    let bindings = bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .allowlist_var("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_type("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_function("^UL.*|JS.*|ul.*|WK.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
