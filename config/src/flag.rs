pub mod fs {
    bitflags::bitflags! {
        pub struct OpenFlag: usize{
            const READ = 1 << 0;
            const WRITE = 1 << 1;
            const CREATE = 1 << 9;
            const TRUNC = 1 << 10;
        }
    }
}
