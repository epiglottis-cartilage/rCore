#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallID {
    Read = 63,
    Write = 64,
    Exit = 93,
    Yield = 124,
    GetTime = 169,
    Sbrk = 214,
}
impl From<usize> for SyscallID {
    fn from(value: usize) -> Self {
        // unimplemented!()
        match value {
            63 => SyscallID::Read,
            64 => SyscallID::Write,
            93 => SyscallID::Exit,
            124 => SyscallID::Yield,
            169 => SyscallID::GetTime,
            214 => SyscallID::Sbrk,
            _ => panic!("Invalid SyscallID: {}", value), // Handle invalid values
        }
    }
}
