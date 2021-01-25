fn main() -> shadow_rs::SdResult<()> {
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    {
        use std::{env, path::PathBuf};

        let path = if cfg!(target_os = "macos") {
            "src/modules/utils/mem_info/mac/bindings-wrapper.h"
        } else {
            "src/modules/utils/mem_info/freebsd/bindings-wrapper.h"
        };

        // There's a mach crate but it doesn't have the require bindings
        println!("cargo:rerun-if-changed={}", path);

        let bindings = bindgen::Builder::default()
            .header(path)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .derive_default(true);

        let bindings = if cfg!(target_os = "macos") {
            bindings
                .whitelist_function("host_statistics64")
                .whitelist_function("mach_host_self")
                .whitelist_type("vm_statistics64")
                .whitelist_var("HOST_VM_INFO64")
                .whitelist_var("KERN_SUCCESS")
                .whitelist_var("STARSHIP_HOST_VM_INFO64_COUNT")
        } else {
            bindings
                .whitelist_type("xswdev")
                .whitelist_var("XSWDEV_VERSION")
        };

        let bindings = bindings.generate().expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("starship-bindings.rs"))
            .expect("Couldn't write bindings!");
    }
    shadow_rs::new()
}
