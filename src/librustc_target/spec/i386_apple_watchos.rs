use crate::spec::{LinkerFlavor, Target, TargetOptions, TargetResult};
use super::apple_ios_base::{opts, Arch};

// https://github.com/apple/swift/blob/fbb7c242f579164afbe4dea09ec3336c9dc171f8/utils/build-script-impl#L631
pub fn target() -> TargetResult {
    let base = opts(Arch::I386_watchos)?;
    Ok(Target {
        llvm_target: "i386-apple-watchos2.0".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:o-p:32:32-f64:32:64-f80:128-n8:16:32-S128".to_string(),
        arch: "x86".to_string(),
        target_os: "watchos".to_string(),
        target_env: String::new(),
        target_vendor: "apple".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            max_atomic_width: Some(64),
            stack_probes: true,
            .. base
        }
    })
}
