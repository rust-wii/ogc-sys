#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use voladdress::{Safe, VolAddress};

pub use types::{Clear, Control, Status};

const BASE: usize = 0xCC00_0000;

const STATUS_REGISTER: VolAddress<Status, Safe, Safe> = unsafe { VolAddress::new(BASE) };

const CONTROL_REGISTER: VolAddress<Control, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x2) };

const CLEAR_REGISTER: VolAddress<Clear, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x4) };

const PERFORMANCE_SELECT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x6) };

const TOKEN: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0xE) };

const BOUNDING_BOX_LEFT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x10) };

const BOUNDING_BOX_RIGHT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x12) };

const BOUNDING_BOX_TOP: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x14) };

const BOUNDING_BOX_BOTTOM: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x16) };

const FIFO_BASE_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x20) };

const FIFO_BASE_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x22) };

const FIFO_END_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x24) };

const FIFO_END_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x26) };

const FIFO_HIGH_WATERMARK_ADDRESS_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x28) };

const FIFO_HIGH_WATERMARK_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2A) };

const FIFO_LOW_WATERMARK_ADDRESS_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2C) };

const FIFO_LOW_WATERMARK_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x2E) };

const FIFO_READ_WRITE_DISTANCE_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x30) };

const FIFO_READ_WRITE_DISTANCE_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x32) };

const FIFO_WRITE_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x34) };

const FIFO_WRITE_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x36) };

const FIFO_READ_ADDRESS_LOW: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x38) };

const FIFO_READ_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(BASE + 0x3A) };

const FIFO_BREAKPOINT_ADDRESS_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3C) };

const FIFO_BREAKPOINT_ADDRESS_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x3E) };

const TRANSFORM_RASTER_BUSY_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x40) };

const TRANSFORM_RASTER_BUSY_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x42) };

const TRANSFORM_CLOCK_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x44) };

const TRANSFORM_CLOCK_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x46) };

const TRANSFORM_WAIT_IN_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x48) };

const TRANSFORM_WAIT_IN_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4A) };

const TRANSFORM_WAIT_OUT_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4C) };

const TRANSFORM_WAIT_OUT_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x4E) };

const VERTEX_CACHE_CHECK_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x50) };

const VERTEX_CACHE_CHECK_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x52) };

const VERTEX_CACHE_MISS_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x54) };

const VERTEX_CACHE_MISS_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x56) };

const VERTEX_CACHE_STALL_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x58) };

const VERTEX_CACHE_STALL_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x5A) };

const CLOCKS_PER_VERTEX_IN_COUNT_LOW: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x60) };

const CLOCKS_PER_VERTEX_IN_COUNT_HIGH: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x62) };

const CLOCKS_PER_VERTEX_OUT_COUNT: VolAddress<u16, Safe, Safe> =
    unsafe { VolAddress::new(BASE + 0x64) };

pub(crate) mod types {
    use bit_field::BitField;

    use super::{CLEAR_REGISTER, CONTROL_REGISTER, STATUS_REGISTER};

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Status(u16);

    impl Status {
        pub const fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            STATUS_REGISTER.read()
        }

        pub fn write(self) {
            STATUS_REGISTER.write(self);
        }

        pub fn overflow(self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_overflow(mut self, has_overflowed: bool) -> Self {
            self.0.set_bit(0, has_overflowed);
            self
        }

        pub fn underflow(self) -> bool {
            self.0.get_bit(1)
        }

        pub fn with_underflow(mut self, has_underflowed: bool) -> Self {
            self.0.set_bit(1, has_underflowed);
            self
        }

        pub fn read_idle(self) -> bool {
            self.0.get_bit(2)
        }

        pub fn with_read_idle(mut self, is_idle: bool) -> Self {
            self.0.set_bit(2, is_idle);
            self
        }

        pub fn command_idle(self) -> bool {
            self.0.get_bit(3)
        }

        pub fn with_command_idle(mut self, is_idle: bool) -> Self {
            self.0.set_bit(3, is_idle);
            self
        }

        pub fn breakpoint(self) -> bool {
            self.0.get_bit(4)
        }

        pub fn with_breakpoint(mut self, breakpoint_hit: bool) -> Self {
            self.0.set_bit(4, breakpoint_hit);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Control(u16);

    impl Control {
        pub const fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            CONTROL_REGISTER.read()
        }

        pub fn write(self) {
            CONTROL_REGISTER.write(self);
        }

        pub fn read_enable(&self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_read_enable(mut self, read_enable: bool) -> Self {
            self.0.set_bit(0, read_enable);
            self
        }

        pub fn breakpoint_enable(&self) -> bool {
            self.0.get_bit(1)
        }

        pub fn with_breakpoint_enable(mut self, breakpoint_enable: bool) -> Self {
            self.0.set_bit(1, breakpoint_enable);
            self
        }

        pub fn overflow_interrupt_enable(&self) -> bool {
            self.0.get_bit(2)
        }

        pub fn with_overflow_interrupt_enable(mut self, overflow_interrupt_enable: bool) -> Self {
            self.0.set_bit(2, overflow_interrupt_enable);
            self
        }

        pub fn underflow_interrupt_enable(&self) -> bool {
            self.0.get_bit(3)
        }

        pub fn with_underflow_interrupt_enable(mut self, underflow_interrupt_enable: bool) -> Self {
            self.0.set_bit(3, underflow_interrupt_enable);
            self
        }

        pub fn link_enable(&self) -> bool {
            self.0.get_bit(4)
        }

        pub fn with_link_enable(mut self, link_enable: bool) -> Self {
            self.0.set_bit(4, link_enable);
            self
        }

        pub fn breakpoint_interrupt_enable(&self) -> bool {
            self.0.get_bit(5)
        }

        pub fn with_breakpoint_interrupt_enable(
            mut self,
            breakpoint_interrupt_enable: bool,
        ) -> Self {
            self.0.set_bit(5, breakpoint_interrupt_enable);
            self
        }
    }

    #[repr(transparent)]
    #[derive(Debug, Copy, Clone)]
    pub struct Clear(u16);

    impl Clear {
        pub const fn new() -> Self {
            Self(0)
        }

        pub fn read() -> Self {
            CLEAR_REGISTER.read()
        }

        pub fn write(self) {
            CLEAR_REGISTER.write(self);
        }

        pub fn clear_overflow(&self) -> bool {
            self.0.get_bit(0)
        }

        pub fn with_clear_overflow(mut self, clear_overflow: bool) -> Self {
            self.0.set_bit(0, clear_overflow);
            self
        }

        pub fn clear_underflow(&self) -> bool {
            self.0.get_bit(1)
        }

        pub fn with_clear_underflow(mut self, clear_underflow: bool) -> Self {
            self.0.set_bit(1, clear_underflow);
            self
        }

        pub fn clear_metrics(&self) -> bool {
            self.0.get_bit(2)
        }

        pub fn with_clear_metrics(mut self, clear_metrics: bool) -> Self {
            self.0.set_bit(2, clear_metrics);
            self
        }
    }
}
