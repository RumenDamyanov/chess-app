#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running chess-app");
}

#[cfg(test)]
mod tests {
    #[test]
    fn package_version_is_the_app_version() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.0.1");
    }
}
