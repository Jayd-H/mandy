fn main() {
    println!("cargo:warning=BUILD.RS IS RUNNING");

    if cfg!(target_os = "windows") {
        let target = std::env::var("CARGO_BIN_NAME").unwrap_or_default();
        println!("cargo:warning=Building target: {}", target);

        let mut res = winres::WindowsResource::new();

        if target == "mandy-installer" {
            println!("cargo:warning=Setting manifest for installer");
            res.set_manifest_file("installer.manifest");
        }

        if std::path::Path::new("icon.ico").exists() {
            println!("cargo:warning=Found icon.ico, embedding it");
            res.set_icon("icon.ico");
        } else {
            println!("cargo:warning=No icon.ico found");
        }

        if target == "mandy-installer" || std::path::Path::new("icon.ico").exists() {
            println!("cargo:warning=Compiling resources");
            match res.compile() {
                Ok(_) => println!("cargo:warning=Resources compiled successfully"),
                Err(e) => println!("cargo:warning=Failed to compile resources: {}", e),
            }
        }
    }
}
