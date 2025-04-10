use std::fs::{File, read_dir};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=/user/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    println!("cargo:rustc-env=LOG=DEBUG");
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<()> {
    let mut apps: Vec<_> = read_dir("../user/")
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|dir| dir.file_type().is_ok_and(|file_type| file_type.is_dir()))
        .filter_map(|dir| dir.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .collect();
    apps.sort();

    let mut f = File::create("src/app.rs").unwrap();
    writeln!(f, "use include_bytes_aligned::*;").unwrap();
    writeln!(f, "pub const APP: [&str; {}] = [", apps.len(),).unwrap();
    for app in &apps {
        writeln!(f, "    \"{0}\",", app).unwrap();
    }
    writeln!(f, "];").unwrap();

    // writeln!(f, "#[repr(C, align(4))]").unwrap();
    writeln!(f, "pub const APP_DATA: [&[u8]; {}] = [", apps.len(),).unwrap();
    for app in &apps {
        writeln!(
            f,
            "    include_bytes_aligned!(4, \"../../{}{}\"),",
            TARGET_PATH, app
        )
        .unwrap();
    }
    writeln!(f, "];").unwrap();

    Ok(())
}
