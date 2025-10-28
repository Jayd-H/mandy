fn main() {
    if cfg!(target_os = "windows") {
        let target = std::env::var("CARGO_BIN_NAME").unwrap_or_default();

        let mut res = winres::WindowsResource::new();

        if target == "mandy-installer" {
            res.set_manifest_file("installer.manifest");
        }

        if std::path::Path::new("icon.ico").exists() {
            res.set_icon("icon.ico");
        }

        if target == "mandy-installer" || std::path::Path::new("icon.ico").exists() {
            res.compile().unwrap();
        }
    }
}
