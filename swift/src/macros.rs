/// Passes assembly into branches based on the target architecture.
///
/// This is used for handling cases of the calling convention like `self`
/// arguments.
macro_rules! arch_asm {
    ($($arch:expr => { $($asm:tt)+ })+) => {
        cfg_if::cfg_if! {
            $(if #[cfg(all(feature = "asm", target_arch = $arch))] {
                asm!($($asm)+);
            } else)+ {
                panic!("This function is not implemented due to lack of inline assembly");
            }
        }
    };
}
