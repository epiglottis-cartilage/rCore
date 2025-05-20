#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalID {
    DEF = 0, // Default signal handling
    HUP = 1,
    INT = 2,
    QUIT = 3,
    ILL = 4,
    TRAP = 5,
    ABRT = 6,
    BUS = 7,
    FPE = 8,
    KILL = 9,
    USR1 = 10,
    SEGV = 11,
    USR2 = 12,
    PIPE = 13,
    ALRM = 14,
    TERM = 15,
    STKFLT = 16,
    CHLD = 17,
    CONT = 18,
    STOP = 19,
    TSTP = 20,
    TTIN = 21,
    TTOU = 22,
    URG = 23,
    XCPU = 24,
    XFSZ = 25,
    VTALRM = 26,
    PROF = 27,
    WINCH = 28,
    IO = 29,
    PWR = 30,
    SYS = 31,
}
impl From<u32> for SignalID {
    fn from(value: u32) -> Self {
        assert!(value < SIG_NUM as u32);
        unsafe { core::mem::transmute(value) }
    }
}
impl From<usize> for SignalID {
    fn from(value: usize) -> Self {
        assert!(value < SIG_NUM);
        unsafe { core::mem::transmute(value as u32) }
    }
}
impl SignalID {
    pub fn job_of_kernel(&self) -> bool {
        match self {
            Self::DEF => true,
            Self::KILL => true,
            Self::STOP => true,
            Self::CONT => true,
            _ => false,
        }
    }
}

pub const SIG_NUM: usize = 32;

/// Action for a signal
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct SignalAction {
    pub handler: usize,
    pub mask: SignalFlags,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SignalFlags: u32 {
        const DEF = 1 << SignalID::DEF as u32; // Default signal handling
        const HUP = 1 << SignalID::HUP as u32;
        const INT = 1 << SignalID::INT as u32;
        const QUIT = 1 << SignalID::QUIT as u32;
        const ILL = 1 << SignalID::ILL as u32;
        const TRAP = 1 << SignalID::TRAP as u32;
        const ABRT = 1 << SignalID::ABRT as u32;
        const BUS = 1 << SignalID::BUS as u32;
        const FPE = 1 << SignalID::FPE as u32;
        const KILL = 1 << SignalID::KILL as u32;
        const USR1 = 1 << SignalID::USR1 as u32;
        const SEGV = 1 << SignalID::SEGV as u32;
        const USR2 = 1 << SignalID::USR2 as u32;
        const PIPE = 1 << SignalID::PIPE as u32;
        const ALRM = 1 << SignalID::ALRM as u32;
        const TERM = 1 << SignalID::TERM as u32;
        const STKFLT = 1 << SignalID::STKFLT as u32;
        const CHLD = 1 << SignalID::CHLD as u32;
        const CONT = 1 << SignalID::CONT as u32;
        const STOP = 1 << SignalID::STOP as u32;
        const TSTP = 1 << SignalID::TSTP as u32;
        const TTIN = 1 << SignalID::TTIN as u32;
        const TTOU = 1 << SignalID::TTOU as u32;
        const URG = 1 << SignalID::URG as u32;
        const XCPU = 1 << SignalID::XCPU as u32;
        const XFSZ = 1 << SignalID::XFSZ as u32;
        const VTALRM = 1 << SignalID::VTALRM as u32;
        const PROF = 1 << SignalID::PROF as u32;
        const WINCH = 1 << SignalID::WINCH as u32;
        const IO = 1 << SignalID::IO as u32;
        const PWR = 1 << SignalID::PWR as u32;
        const SYS = 1 << SignalID::SYS as u32;
    }
}
impl SignalFlags {
    pub fn check_error(&self) -> Option<(i32, &'static str)> {
        if self.contains(Self::INT) {
            Some((-2, "Killed, SIGINT=2"))
        } else if self.contains(Self::ILL) {
            Some((-4, "Illegal Instruction, SIGILL=4"))
        } else if self.contains(Self::ABRT) {
            Some((-6, "Aborted, SIGABRT=6"))
        } else if self.contains(Self::FPE) {
            Some((-8, "Erroneous Arithmetic Operation, SIGFPE=8"))
        } else if self.contains(Self::KILL) {
            Some((-9, "Killed, SIGKILL=9"))
        } else if self.contains(Self::SEGV) {
            Some((-11, "Segmentation Fault, SIGSEGV=11"))
        } else {
            //println!("[K] signalflags check_error  {:?}", self);
            None
        }
    }
}

impl Default for SignalAction {
    fn default() -> Self {
        Self {
            handler: 0,
            mask: SignalFlags::QUIT | SignalFlags::TRAP,
        }
    }
}

#[derive(Clone)]
pub struct SignalActions {
    pub table: [SignalAction; SIG_NUM],
}

impl Default for SignalActions {
    fn default() -> Self {
        Self {
            table: core::array::from_fn(|_| SignalAction::default()),
        }
    }
}
