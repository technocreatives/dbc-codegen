// Generated code!
#![no_std]
#![allow(unused, clippy::let_and_return, clippy::eq_op)]

//! Message definitions from file `"example.dbc"`
//!
//! - Version: `Version("42")`

use bitsh::Pack;

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Messages {
    /// Foo
    Foo(Foo),
    /// Bar
    Bar(Bar),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        use core::convert::TryFrom;

        let res = match id {
            256 => Messages::Foo(Foo::try_from(payload)?),
            512 => Messages::Bar(Bar::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// Foo
///
/// - ID: 256 (0x100)
/// - Size: 4 bytes
/// - Transmitter: Lorem
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Foo {
    raw: [u8; 4],
}

impl Foo {
    pub const MESSAGE_ID: u32 = 256;

    /// Construct new Foo from values
    pub fn new(voltage: f64, current: f64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_voltage(voltage)?;
        res.set_current(current)?;
        Ok(res)
    }

    /// Voltage
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.000976562
    /// - Offset: 0
    /// - Min: 0
    /// - Max: 63.9990234375
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn voltage(&self) -> f64 {
        let signal = u16::unpack_le_bits(&self.raw, 16, 16);

        let factor = 0.000976562_f64;
        let offset = 0_f64;
        (signal as f64) * factor + offset
    }

    /// Set value of Voltage
    #[inline(always)]
    pub fn set_voltage(&mut self, value: f64) -> Result<(), CanError> {
        let factor = 0.000976562_f64;
        let offset = 0_f64;
        let value = ((value - offset) / factor) as u16;

        let start_bit = 16;
        let bits = 16;
        value.pack_le_bits(&mut self.raw, start_bit, bits);
        Ok(())
    }

    /// Current
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.0625
    /// - Offset: 0
    /// - Min: -2048
    /// - Max: 2047.9375
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn current(&self) -> f64 {
        let signal = i16::unpack_le_bits(&self.raw, 0, 16);

        let factor = 0.0625_f64;
        let offset = 0_f64;
        (signal as f64) * factor + offset
    }

    /// Set value of Current
    #[inline(always)]
    pub fn set_current(&mut self, value: f64) -> Result<(), CanError> {
        let factor = 0.0625_f64;
        let offset = 0_f64;
        let value = ((value - offset) / factor) as i16;

        let start_bit = 0;
        let bits = 16;
        value.pack_le_bits(&mut self.raw, start_bit, bits);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Foo {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

/// Bar
///
/// - ID: 512 (0x200)
/// - Size: 8 bytes
/// - Transmitter: Ipsum
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Bar {
    raw: [u8; 8],
}

impl Bar {
    pub const MESSAGE_ID: u32 = 512;

    /// Construct new Bar from values
    pub fn new(one: u8, two: f64, three: u8, four: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_one(one)?;
        res.set_two(two)?;
        res.set_three(three)?;
        res.set_four(four)?;
        Ok(res)
    }

    /// One
    ///
    /// - Start bit: 15
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Min: 0
    /// - Max: 3
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn one(&self) -> u8 {
        let signal = u8::unpack_be_bits(&self.raw, (15 - (2 - 1)), 2);

        signal
    }

    /// Set value of One
    #[inline(always)]
    pub fn set_one(&mut self, value: u8) -> Result<(), CanError> {
        let start_bit = 15;
        let bits = 2;
        value.pack_be_bits(&mut self.raw, start_bit, bits);
        Ok(())
    }

    /// Two
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 0.39
    /// - Offset: 0
    /// - Min: 0
    /// - Max: 100
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    /// - Unit: "%"
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn two(&self) -> f64 {
        let signal = u8::unpack_be_bits(&self.raw, (7 - (8 - 1)), 8);

        let factor = 0.39_f64;
        let offset = 0_f64;
        (signal as f64) * factor + offset
    }

    /// Set value of Two
    #[inline(always)]
    pub fn set_two(&mut self, value: f64) -> Result<(), CanError> {
        let factor = 0.39_f64;
        let offset = 0_f64;
        let value = ((value - offset) / factor) as u8;

        let start_bit = 7;
        let bits = 8;
        value.pack_be_bits(&mut self.raw, start_bit, bits);
        Ok(())
    }

    /// Three
    ///
    /// - Start bit: 13
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Min: 0
    /// - Max: 7
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn three(&self) -> u8 {
        let signal = u8::unpack_be_bits(&self.raw, (13 - (3 - 1)), 3);

        signal
    }

    /// Set value of Three
    #[inline(always)]
    pub fn set_three(&mut self, value: u8) -> Result<(), CanError> {
        let start_bit = 13;
        let bits = 3;
        value.pack_be_bits(&mut self.raw, start_bit, bits);
        Ok(())
    }

    /// Four
    ///
    /// - Start bit: 10
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Min: 0
    /// - Max: 3
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn four(&self) -> u8 {
        let signal = u8::unpack_be_bits(&self.raw, (10 - (2 - 1)), 2);

        signal
    }

    /// Set value of Four
    #[inline(always)]
    pub fn set_four(&mut self, value: u8) -> Result<(), CanError> {
        let start_bit = 10;
        let bits = 2;
        value.pack_be_bits(&mut self.raw, start_bit, bits);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Bar {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

/// This is just to make testing easier
fn main() {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    InvalidPayloadSize,
}
