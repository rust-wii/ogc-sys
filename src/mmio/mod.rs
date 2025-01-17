pub mod cp;
pub mod di;
pub mod dsp;
pub mod mi;
pub mod pe;
pub mod pi;

/// Serial Interface Helper Types and MMIO
///
/// This is used to interact with various serial devices, including Gamecube Controllers, GBA
/// and GBASP when using a specific link cable. This can also be used to make custom peripherals
/// with the serial interface command buffers.
pub mod serial_interface;

pub mod vi;
