#[cfg(target_os = "redox")]
pub fn get_display_size() -> (i32, i32) {
    match File::open("display:") {
        Ok(display) => {
            let path = display.path().map(|path| path.into_os_string().into_string().unwrap_or(String::new())).unwrap_or(String::new());
            let res = path.split(":").nth(1).unwrap_or("");
            let width = res.split("/").nth(0).unwrap_or("").parse::<i32>().unwrap_or(0);
            let height = res.split("/").nth(1).unwrap_or("").parse::<i32>().unwrap_or(0);
            (width, height)
        },
        Err(err) => panic!("launcher: failed to get display size: {}", err)
    }
}

#[cfg(not(target_os = "redox"))]
pub fn get_display_size() -> (i32, i32) {
    panic!("launcher: failed to get display size")
}
