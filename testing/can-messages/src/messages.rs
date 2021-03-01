// Generated code!
#![no_std]
#![allow(unused, clippy::let_and_return, clippy::eq_op)]

//! Message definitions from file `"example.dbc"`
//!
//! - Version: `Version("43")`

#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};
use bitsh::Pack;
use bitvec::prelude::{BitField, BitStore, BitView, LocalBits};

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
    pub fn new(voltage: f32, current: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_voltage(voltage)?;
        res.set_current(current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    /// Voltage
    ///
    /// - Min: 0
    /// - Max: 63.9990234375
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn voltage(&self) -> f32 {
        self.voltage_raw()
    }

    /// Get raw value of Voltage
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.000976562
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<LocalBits>()[16..32].load_le::<u16>();

        let factor = 0.000976562_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Voltage
    #[inline(always)]
    pub fn set_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 63.9990234375_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        let factor = 0.000976562_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<LocalBits>()[16..32].store_le(value);
        Ok(())
    }

    /// Current
    ///
    /// - Min: -2048
    /// - Max: 2047.9375
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn current(&self) -> f32 {
        self.current_raw()
    }

    /// Get raw value of Current
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.0625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<LocalBits>()[0..16].load_le::<u16>();

        let factor = 0.0625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Current
    #[inline(always)]
    pub fn set_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -2048_f32 || 2047.9375_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        let factor = 0.0625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<LocalBits>()[0..16].store_le(value);
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

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Foo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let voltage = 0_f32;
        let current = -2048_f32;
        Foo::new(voltage, current).map_err(|_| arbitrary::Error::IncorrectFormat)
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
    pub fn new(one: u8, two: f32, three: u8, four: u8, xtype: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_one(one)?;
        res.set_two(two)?;
        res.set_three(three)?;
        res.set_four(four)?;
        res.set_xtype(xtype)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    /// One
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn one(&self) -> u8 {
        self.one_raw()
    }

    /// Get raw value of One
    ///
    /// - Start bit: 15
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn one_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<LocalBits>()[15..17].load_be::<u8>();

        signal
    }

    /// Set value of One
    #[inline(always)]
    pub fn set_one(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        self.raw.view_bits_mut::<LocalBits>()[15..17].store_be(value);
        Ok(())
    }

    /// Two
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn two(&self) -> f32 {
        self.two_raw()
    }

    /// Get raw value of Two
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 0.39
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn two_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<LocalBits>()[7..15].load_be::<u8>();

        let factor = 0.39_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Two
    #[inline(always)]
    pub fn set_two(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        let factor = 0.39_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<LocalBits>()[7..15].store_be(value);
        Ok(())
    }

    /// Three
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn three(&self) -> BarThree {
        match self.three_raw() {
            0 => BarThree::Off,
            1 => BarThree::On,
            2 => BarThree::Oner,
            3 => BarThree::Onest,
            x => BarThree::Other(x),
        }
    }

    /// Get raw value of Three
    ///
    /// - Start bit: 17
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn three_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<LocalBits>()[17..20].load_be::<u8>();

        signal
    }

    /// Set value of Three
    #[inline(always)]
    pub fn set_three(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        self.raw.view_bits_mut::<LocalBits>()[17..20].store_be(value);
        Ok(())
    }

    /// Four
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn four(&self) -> BarFour {
        match self.four_raw() {
            0 => BarFour::Off,
            1 => BarFour::On,
            2 => BarFour::Oner,
            3 => BarFour::Onest,
            x => BarFour::Other(x),
        }
    }

    /// Get raw value of Four
    ///
    /// - Start bit: 20
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn four_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<LocalBits>()[20..22].load_be::<u8>();

        signal
    }

    /// Set value of Four
    #[inline(always)]
    pub fn set_four(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        self.raw.view_bits_mut::<LocalBits>()[20..22].store_be(value);
        Ok(())
    }

    /// Type
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn xtype(&self) -> BarType {
        match self.xtype_raw() {
            false => BarType::X0off,
            true => BarType::X1on,
            x => BarType::Other(x),
        }
    }

    /// Get raw value of Type
    ///
    /// - Start bit: 30
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn xtype_raw(&self) -> bool {
        let signal = u8::unpack_be_bits(&self.raw, (30 - (1 - 1)), 1);

        signal == 1
    }

    /// Set value of Type
    #[inline(always)]
    pub fn set_xtype(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        let start_bit = 30;
        let bits = 1;
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

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Bar {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let one = u.int_in_range(0..=3)?;
        let two = 0_f32;
        let three = u.int_in_range(0..=7)?;
        let four = u.int_in_range(0..=3)?;
        let xtype = u.int_in_range(0..=1)? == 1;
        Bar::new(one, two, three, four, xtype).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for Three
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BarThree {
    Off,
    On,
    Oner,
    Onest,
    Other(u8),
}
/// Defined values for Four
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BarFour {
    Off,
    On,
    Oner,
    Onest,
    Other(u8),
}
/// Defined values for Type
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BarType {
    X0off,
    X1on,
    Other(bool),
}

/// This is just to make testing easier
fn main() {}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
}
