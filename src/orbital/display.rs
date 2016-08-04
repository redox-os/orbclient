use std::fs::File;

pub fn get_display_size() -> Result<(u32, u32), String> {
    match File::open("display:") {
        Ok(display) => {
            let path = display.path().map(|path| path.into_os_string().into_string().unwrap_or(String::new())).unwrap_or(String::new());
            let res = path.split(":").nth(1).unwrap_or("");
            let width = res.split("/").nth(0).unwrap_or("").parse::<u32>().unwrap_or(0);
            let height = res.split("/").nth(1).unwrap_or("").parse::<u32>().unwrap_or(0);
            Ok((width, height))
        },
        Err(err) => Err(format!("{}", err))
    }
}
