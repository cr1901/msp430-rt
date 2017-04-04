#[cfg(feature = "linker-script")]
mod imp {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    pub fn main() {
        // Put the linker script somewhere the linker can find it
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("link.x"))
            .unwrap()
            .write_all(include_bytes!("link.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());

        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=link.x");
    }
}

#[cfg(not(feature = "linker-script"))]
mod imp {
    pub fn main() {}
}

fn main() {
    imp::main();
}