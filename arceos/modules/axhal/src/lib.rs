//! [ArceOS] hardware abstraction layer, provides unified APIs for
//! platform-specific operations.
//!
//! It does the bootstrapping and initialization process for the specified
//! platform, and provides useful operations on the hardware.
//!
//! Currently supported platforms (specify by cargo features):
//!
//! - `x86-pc`: Standard PC with x86_64 ISA.
//! - `riscv64-qemu-virt`: QEMU virt machine with RISC-V ISA.
//! - `aarch64-qemu-virt`: QEMU virt machine with AArch64 ISA.
//! - `aarch64-raspi`: Raspberry Pi with AArch64 ISA.
//! - `dummy`: If none of the above platform is selected, the dummy platform
//!    will be used. In this platform, most of the operations are no-op or
//!    `unimplemented!()`. This platform is mainly used for [cargo test].
//!
//! # Cargo Features
//!
//! - `smp`: Enable SMP (symmetric multiprocessing) support.
//! - `fp_simd`: Enable floating-point and SIMD support.
//! - `paging`: Enable page table manipulation.
//! - `irq`: Enable interrupt handling support.
//!
//! [ArceOS]: https://github.com/arceos-org/arceos
//! [cargo test]: https://doc.rust-lang.org/cargo/guide/tests.html

#![no_std]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(const_option)]
#![feature(doc_auto_cfg)]
#![feature(const_mut_refs)]

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[allow(unused_imports)]
#[macro_use]
extern crate memory_addr;

mod platform;

#[macro_use]
pub mod trap;

pub mod arch;
pub mod cpu;
pub mod mem;
pub mod time;

#[cfg(feature = "tls")]
pub mod tls;

#[cfg(feature = "irq")]
pub mod irq;

#[cfg(feature = "paging")]
pub mod paging;

/// Console input and output.
pub mod console {
    pub use super::platform::console::*;

    // Define ANSI color codes as constants for easy maintenance
    const RESET_CODE: &[u8] = b"\x1b[0m";
    const COLOR_CODE: &[u8] = b"\x1b[34m";

    /// Write a slice of bytes to the console.
    pub fn write_bytes(bytes: &[u8]) {
        for &c in COLOR_CODE {
            putchar(c);
        }

        // Write each byte in the given slice
        for &c in bytes {
            putchar(c);
        }

        // Reset the color
        for &c in RESET_CODE {
            putchar(c);
        }
    }
}

/// Miscellaneous operation, e.g. terminate the system.
pub mod misc;

/// Multi-core operations.
#[cfg(feature = "smp")]
pub mod mp {
    pub use super::platform::mp::*;
}

pub use self::platform::platform_init;

#[cfg(feature = "smp")]
pub use self::platform::platform_init_secondary;
