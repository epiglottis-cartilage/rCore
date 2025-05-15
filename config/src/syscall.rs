#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallID {
    Open = 56,
    Close = 57,
    Pipe = 59,
    Read = 63,
    Write = 64,
    Exit = 93,
    Yield = 124,
    GetTime = 169,
    GetPid = 172,
    Sbrk = 214,
    Fork = 220,
    Exec = 221,
    WaitPid = 260,
}
// impl From<usize> for SyscallID {
//     fn from(value: usize) -> Self {
//         match value {
//             63 => SyscallID::Read,
//             64 => SyscallID::Write,
//             93 => SyscallID::Exit,
//             124 => SyscallID::Yield,
//             169 => SyscallID::GetTime,
//             214 => SyscallID::Sbrk,
//             220 => SyscallID::Fork,
//             221 => SyscallID::Exec,
//             260 => SyscallID::WaitPid,
//             _ => panic!("Invalid SyscallID: {}", value), // Handle invalid values
//         }
//     }
// }

#[test]
fn test_argument() {
    let argv: &[&str] = &["hello", "world", "rCore", "🥵🥵🥵🥵"];
    let argv_packed: Argument<&str> = argv.into();
    let os_recv =
        unsafe { *(&argv_packed as *const Argument<&str> as *const Argument<Argument<u8>>) };

    for (recv, send) in
        unsafe { core::slice::from_raw_parts(os_recv.user_ptr as *const Argument<u8>, os_recv.len) }
            .iter()
            .map(|x| unsafe {
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    x.user_ptr as *const u8,
                    x.len,
                ))
            })
            .zip(argv.iter())
    {
        assert_eq!(recv, *send);
    }
}
