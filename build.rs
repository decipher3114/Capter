fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icons/windows/icon.ico");
        res.compile().unwrap();
    }
}