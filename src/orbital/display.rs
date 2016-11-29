use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use super::syscall;

pub fn get_display_size() -> Result<(u32, u32), String> {
    let display_path = try!(env::var("DISPLAY").or(Err("DISPLAY not set")));
    match File::open(&display_path) {
        Ok(display) => {
            let mut buf: [u8; 4096] = [0; 4096];
            let count = syscall::fpath(display.as_raw_fd() as usize, &mut buf).map_err(|err| format!("{}", err))?;
            let path = unsafe { String::from_utf8_unchecked(Vec::from(&buf[..count])) };
            let res = path.split(":").nth(1).unwrap_or("");
            let width = res.split("/").nth(1).unwrap_or("").parse::<u32>().unwrap_or(0);
            let height = res.split("/").nth(2).unwrap_or("").parse::<u32>().unwrap_or(0);
            Ok((width, height))
        },
        Err(err) => Err(format!("{}", err))
    }
}
