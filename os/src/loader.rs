//! Loading user applications into memory

// This file is generated by `build.rs`
include!("app.rs");

/// Get the total number of applications.
#[inline(always)]
pub fn app_number() -> usize {
    APP.len()
}

/// get applications data
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    log::debug!("app_address: {:X?}", APP_DATA[app_id].as_ptr() as usize);
    assert!(app_id < app_number());
    return APP_DATA[app_id];
}

#[allow(unused)]
///get app data from name
pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    APP.binary_search(&name).ok().map(get_app_data)
}
///list all apps
pub fn list_apps() {
    log::debug!("/**** APPS ****\n{:#?}\n", APP);
}
