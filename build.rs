fn main() -> shadow_rs::SdResult<()> {
    #[cfg(target_os = "macos")]
    {
        use std::{env, path::PathBuf};

        // There's a mach crate but it doesn't have the require bindings
        println!("cargo:rerun-if-changed=src/modules/utils/mem_info/mac/macos-bindings-wrapper.h");

        let bindings = bindgen::Builder::default()
            .header("src/modules/utils/mem_info/mac/macos-bindings-wrapper.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .whitelist_function("host_statistics64")
            .whitelist_function("mach_host_self")
            .whitelist_type("vm_statistics64")
            .whitelist_var("HOST_VM_INFO64")
            .whitelist_var("KERN_SUCCESS")
            .whitelist_var("STARSHIP_HOST_VM_INFO64_COUNT")
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("mach-bindings.rs"))
            .expect("Couldn't write bindings!");
    }
    shadow_rs::new()
}
