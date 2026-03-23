fn main() {
    #[cfg(target_os = "windows")]
    {
        let icon = "resources/windows/app-icon.ico";
        let icon_path = std::path::Path::new(icon);

        if icon_path.exists() {
            println!("cargo:rerun-if-changed={}", icon);

            let mut res = winresource::WindowsResource::new();
            res.set_icon(icon);
            res.set("FileDescription", "LILYGO Spark NT");
            res.set("ProductName", "LILYGO Spark NT");

            if let Err(e) = res.compile() {
                eprintln!("cargo:warning=Failed to set Windows resource: {}", e);
            }
        } else {
            println!("cargo:warning=Windows icon not found at {}, exe will have no icon", icon);
        }
    }
}
