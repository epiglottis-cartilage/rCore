#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallID {
    Read = 63,
    Write = 64,
    Exit = 93,
    Yield = 124,
    GetTime = 169,
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
