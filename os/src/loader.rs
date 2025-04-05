//! Loading user applications into memory

// This file is generated by `build.rs`
include!("app.rs");

/// Get the total number of applications.
#[inline(always)]
pub fn get_num_app() -> usize {
    APP.len()
}

/// get applications data
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    use crate::label::app_address;
    log::info!("loading app[{}]", app_id);
    let app_start = unsafe {
        core::slice::from_raw_parts(app_address as usize as *const usize, get_num_app() + 1)
    };
    assert!(app_id < get_num_app());
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
