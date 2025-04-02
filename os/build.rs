use std::fs::{File, read_dir};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=/user/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    println!("cargo:rustc-env=LOG=TRACE");
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.asm").unwrap();
    let mut apps: Vec<_> = read_dir("../user/")
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|dir| dir.file_type().is_ok_and(|file_type| file_type.is_dir()))
        .filter_map(|dir| dir.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global num_app
num_app:
    .quad {}"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}
