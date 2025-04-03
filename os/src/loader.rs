//! Loading user applications into memory

/// Get the total number of applications.
#[inline(always)]
pub fn get_num_app() -> usize {
    use crate::label::num_app;
    num_app
}

/// get applications data
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    use crate::label::num_app;
    log::info!("loading app[{}]", app_id);
    log::trace!("app_num={} at {:X?}", num_app, core::ptr::addr_of!(num_app));
    let app_start = unsafe {
        core::slice::from_raw_parts(core::ptr::addr_of!(num_app).add(1), get_num_app() + 1)
    };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
