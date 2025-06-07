use std::fs::create_dir_all;

pub fn appdata() -> String {
        #[cfg(unix)]
    let app_data = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");

    return app_data + "\\modmanager2.0";
}

pub fn tmp() -> String {
    let path =  appdata() + "\\tmp";
    create_dir_all(path.clone()).expect("Couldn't create path");
    path
}

pub fn is_java_installed() -> bool {
    let java_home = std::env::var("JAVA_HOME"); // delta target
    return java_home.is_ok();
}