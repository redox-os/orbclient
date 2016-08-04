use super::{init, VIDEO_CTX};

pub fn get_display_size() -> Result<(u32, u32), String> {
    unsafe { init() };
    unsafe { & *VIDEO_CTX }.display_bounds(0)
        .map(|rect| (rect.width(), rect.height()))
        .map_err(|err| format!("{}", err))
}
