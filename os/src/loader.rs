//! Loading user applications into memory

/// Get the total number of applications.
#[inline]
pub fn get_num_app() -> usize {
    use crate::label::num_app;
    num_app
}

/// get applications data
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    use crate::label::num_app;
    let app_start =
        unsafe { core::slice::from_raw_parts((num_app as *const usize).add(1), get_num_app() + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
