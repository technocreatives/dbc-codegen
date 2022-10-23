// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::integer_arithmetic)]

//! Message definitions from file `"example.dbc"`
//!
//! - Version: `Version("43")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Messages {
    /// Foo
    Foo(Foo),
    /// FooInexact
    FooInexact(FooInexact),
    /// Bar
    Bar(Bar),
    /// _4WD
    X4wd(X4wd),
    /// Amet
    Amet(Amet),
    /// Dolor
    Dolor(Dolor),
    /// MultiplexTest
    MultiplexTest(MultiplexTest),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        use core::convert::TryFrom;
        
        let res = match id {
            256 => Messages::Foo(Foo::try_from(payload)?),
            256 => Messages::FooInexact(FooInexact::try_from(payload)?),
            512 => Messages::Bar(Bar::try_from(payload)?),
            768 => Messages::X4wd(X4wd::try_from(payload)?),
            1024 => Messages::Amet(Amet::try_from(payload)?),
            1028 => Messages::Dolor(Dolor::try_from(payload)?),
            200 => Messages::MultiplexTest(MultiplexTest::try_from(payload)?),
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
pub struct Foo {
    raw: [u8; 4],
}

impl Foo {
    pub const MESSAGE_ID: u32 = 256;
    
    pub const VOLTAGE_MIN: f32 = 0_f32;
    pub const VOLTAGE_MAX: f32 = 63.9990234375_f32;
    pub const CURRENT_MIN: f32 = -2048_f32;
    pub const CURRENT_MAX: f32 = 2047.9375_f32;
    
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
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
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
        let value = ((value - offset) / factor).round() as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
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
        let value = ((value - offset) / factor).round() as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Foo {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Foo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Foo")
                .field("voltage", &self.voltage())
                .field("current", &self.current())
            .finish()
        } else {
            f.debug_tuple("Foo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Foo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let voltage = u.float_in_range(0_f32..=63.9990234375_f32)?;
        let current = u.float_in_range(-2048_f32..=2047.9375_f32)?;
        Foo::new(voltage,current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// FooInexact
///
/// - ID: 256 (0x100)
/// - Size: 4 bytes
/// - Transmitter: Test
#[derive(Clone, Copy)]
pub struct FooInexact {
    raw: [u8; 4],
}

impl FooInexact {
    pub const MESSAGE_ID: u32 = 256;
    
    pub const VOLTAGE_MIN: f32 = 0_f32;
    pub const VOLTAGE_MAX: f32 = 655.35_f32;
    pub const CURRENT_MIN: f32 = -327.68_f32;
    pub const CURRENT_MAX: f32 = 327.67_f32;
    
    /// Construct new FooInexact from values
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
    /// - Max: 655.35
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
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Voltage
    #[inline(always)]
    pub fn set_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 655.35_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor).round() as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// Current
    ///
    /// - Min: -327.68
    /// - Max: 327.67
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
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Current
    #[inline(always)]
    pub fn set_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -327.68_f32 || 327.67_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor).round() as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for FooInexact {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for FooInexact {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("FooInexact")
                .field("voltage", &self.voltage())
                .field("current", &self.current())
            .finish()
        } else {
            f.debug_tuple("FooInexact").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for FooInexact {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let voltage = u.float_in_range(0_f32..=655.35_f32)?;
        let current = u.float_in_range(-327.68_f32..=327.67_f32)?;
        FooInexact::new(voltage,current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Bar
///
/// - ID: 512 (0x200)
/// - Size: 8 bytes
/// - Transmitter: Ipsum
#[derive(Clone, Copy)]
pub struct Bar {
    raw: [u8; 8],
}

impl Bar {
    pub const MESSAGE_ID: u32 = 512;
    
    pub const ONE_MIN: u8 = 0_u8;
    pub const ONE_MAX: u8 = 3_u8;
    pub const TWO_MIN: f32 = 0_f32;
    pub const TWO_MAX: f32 = 100_f32;
    pub const THREE_MIN: u8 = 0_u8;
    pub const THREE_MAX: u8 = 7_u8;
    pub const FOUR_MIN: u8 = 0_u8;
    pub const FOUR_MAX: u8 = 3_u8;
    
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
        let signal = self.raw.view_bits::<Msb0>()[8..10].load_be::<u8>();
        
        signal
    }
    
    /// Set value of One
    #[inline(always)]
    pub fn set_one(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        self.raw.view_bits_mut::<Msb0>()[8..10].store_be(value);
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
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();
        
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
        let value = ((value - offset) / factor).round() as u8;
        
        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
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
        let signal = self.raw.view_bits::<Msb0>()[10..13].load_be::<u8>();
        
        match signal {
            0 => BarThree::Off,
            1 => BarThree::On,
            2 => BarThree::Oner,
            3 => BarThree::Onest,
            _ => BarThree::_Other(self.three_raw()),
        }
    }
    
    /// Get raw value of Three
    ///
    /// - Start bit: 13
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn three_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[10..13].load_be::<u8>();
        
        signal
    }
    
    /// Set value of Three
    #[inline(always)]
    pub fn set_three(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        self.raw.view_bits_mut::<Msb0>()[10..13].store_be(value);
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
        let signal = self.raw.view_bits::<Msb0>()[13..15].load_be::<u8>();
        
        match signal {
            0 => BarFour::Off,
            1 => BarFour::On,
            2 => BarFour::Oner,
            3 => BarFour::Onest,
            _ => BarFour::_Other(self.four_raw()),
        }
    }
    
    /// Get raw value of Four
    ///
    /// - Start bit: 10
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn four_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[13..15].load_be::<u8>();
        
        signal
    }
    
    /// Set value of Four
    #[inline(always)]
    pub fn set_four(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 512 });
        }
        self.raw.view_bits_mut::<Msb0>()[13..15].store_be(value);
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
        let signal = self.raw.view_bits::<Msb0>()[25..26].load_be::<u8>();
        
        match signal {
            0 => BarType::X0off,
            1 => BarType::X1on,
            _ => BarType::_Other(self.xtype_raw()),
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
        let signal = self.raw.view_bits::<Msb0>()[25..26].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of Type
    #[inline(always)]
    pub fn set_xtype(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[25..26].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bar {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Bar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Bar")
                .field("one", &self.one())
                .field("two", &self.two())
                .field("three", &self.three())
                .field("four", &self.four())
                .field("xtype", &self.xtype())
            .finish()
        } else {
            f.debug_tuple("Bar").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Bar {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let one = u.int_in_range(0..=3)?;
        let two = u.float_in_range(0_f32..=100_f32)?;
        let three = u.int_in_range(0..=7)?;
        let four = u.int_in_range(0..=3)?;
        let xtype = u.int_in_range(0..=1)? == 1;
        Bar::new(one,two,three,four,xtype).map_err(|_| arbitrary::Error::IncorrectFormat)
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
    _Other(u8),
}

impl Into<u8> for BarThree {
    fn into(self) -> u8 {
        match self {
            BarThree::Off => 0,
            BarThree::On => 1,
            BarThree::Oner => 2,
            BarThree::Onest => 3,
            BarThree::_Other(x) => x,
        }
    }
}

/// Defined values for Four
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BarFour {
    Off,
    On,
    Oner,
    Onest,
    _Other(u8),
}

impl Into<u8> for BarFour {
    fn into(self) -> u8 {
        match self {
            BarFour::Off => 0,
            BarFour::On => 1,
            BarFour::Oner => 2,
            BarFour::Onest => 3,
            BarFour::_Other(x) => x,
        }
    }
}

/// Defined values for Type
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BarType {
    X0off,
    X1on,
    _Other(bool),
}

impl Into<bool> for BarType {
    fn into(self) -> bool {
        match self {
            BarType::X0off => false,
            BarType::X1on => true,
            BarType::_Other(x) => x,
        }
    }
}


/// _4WD
///
/// - ID: 768 (0x300)
/// - Size: 8 bytes
/// - Transmitter: Ipsum
#[derive(Clone, Copy)]
pub struct X4wd {
    raw: [u8; 8],
}

impl X4wd {
    pub const MESSAGE_ID: u32 = 768;
    
    pub const X4DRIVE_MIN: u8 = 0_u8;
    pub const X4DRIVE_MAX: u8 = 7_u8;
    
    /// Construct new _4WD from values
    pub fn new(x4drive: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_x4drive(x4drive)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// _4DRIVE
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn x4drive(&self) -> X4wd4drive {
        let signal = self.raw.view_bits::<Msb0>()[10..13].load_be::<u8>();
        
        match signal {
            0 => X4wd4drive::Off,
            1 => X4wd4drive::X2wd,
            2 => X4wd4drive::X4wd,
            3 => X4wd4drive::All,
            _ => X4wd4drive::_Other(self.x4drive_raw()),
        }
    }
    
    /// Get raw value of _4DRIVE
    ///
    /// - Start bit: 13
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn x4drive_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[10..13].load_be::<u8>();
        
        signal
    }
    
    /// Set value of _4DRIVE
    #[inline(always)]
    pub fn set_x4drive(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 768 });
        }
        self.raw.view_bits_mut::<Msb0>()[10..13].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for X4wd {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for X4wd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("X4wd")
                .field("x4drive", &self.x4drive())
            .finish()
        } else {
            f.debug_tuple("X4wd").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for X4wd {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let x4drive = u.int_in_range(0..=7)?;
        X4wd::new(x4drive).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for _4DRIVE
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum X4wd4drive {
    Off,
    X2wd,
    X4wd,
    All,
    _Other(u8),
}

impl Into<u8> for X4wd4drive {
    fn into(self) -> u8 {
        match self {
            X4wd4drive::Off => 0,
            X4wd4drive::X2wd => 1,
            X4wd4drive::X4wd => 2,
            X4wd4drive::All => 3,
            X4wd4drive::_Other(x) => x,
        }
    }
}


/// Amet
///
/// - ID: 1024 (0x400)
/// - Size: 8 bytes
/// - Transmitter: Sit
#[derive(Clone, Copy)]
pub struct Amet {
    raw: [u8; 8],
}

impl Amet {
    pub const MESSAGE_ID: u32 = 1024;
    
    pub const ONE_MIN: u8 = 0_u8;
    pub const ONE_MAX: u8 = 3_u8;
    pub const TWO_MIN: f32 = 0_f32;
    pub const TWO_MAX: f32 = 100_f32;
    pub const THREE_MIN: u8 = 0_u8;
    pub const THREE_MAX: u8 = 7_u8;
    pub const FOUR_MIN: u8 = 0_u8;
    pub const FOUR_MAX: u8 = 3_u8;
    
    /// Construct new Amet from values
    pub fn new(one: u8, two: f32, three: u8, four: u8, five: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_one(one)?;
        res.set_two(two)?;
        res.set_three(three)?;
        res.set_four(four)?;
        res.set_five(five)?;
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
        let signal = self.raw.view_bits::<Msb0>()[8..10].load_be::<u8>();
        
        signal
    }
    
    /// Set value of One
    #[inline(always)]
    pub fn set_one(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 1024 });
        }
        self.raw.view_bits_mut::<Msb0>()[8..10].store_be(value);
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
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();
        
        let factor = 0.39_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Two
    #[inline(always)]
    pub fn set_two(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 1024 });
        }
        let factor = 0.39_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor).round() as u8;
        
        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
        Ok(())
    }
    
    /// Three
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn three(&self) -> u8 {
        self.three_raw()
    }
    
    /// Get raw value of Three
    ///
    /// - Start bit: 20
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn three_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[19..22].load_be::<u8>();
        
        signal
    }
    
    /// Set value of Three
    #[inline(always)]
    pub fn set_three(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 1024 });
        }
        self.raw.view_bits_mut::<Msb0>()[19..22].store_be(value);
        Ok(())
    }
    
    /// Four
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn four(&self) -> u8 {
        self.four_raw()
    }
    
    /// Get raw value of Four
    ///
    /// - Start bit: 30
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn four_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[25..27].load_be::<u8>();
        
        signal
    }
    
    /// Set value of Four
    #[inline(always)]
    pub fn set_four(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 1024 });
        }
        self.raw.view_bits_mut::<Msb0>()[25..27].store_be(value);
        Ok(())
    }
    
    /// Five
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "boolean"
    /// - Receivers: Dolor
    #[inline(always)]
    pub fn five(&self) -> bool {
        self.five_raw()
    }
    
    /// Get raw value of Five
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn five_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[47..48].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of Five
    #[inline(always)]
    pub fn set_five(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[47..48].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Amet {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Amet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Amet")
                .field("one", &self.one())
                .field("two", &self.two())
                .field("three", &self.three())
                .field("four", &self.four())
                .field("five", &self.five())
            .finish()
        } else {
            f.debug_tuple("Amet").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Amet {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let one = u.int_in_range(0..=3)?;
        let two = u.float_in_range(0_f32..=100_f32)?;
        let three = u.int_in_range(0..=7)?;
        let four = u.int_in_range(0..=3)?;
        let five = u.int_in_range(0..=1)? == 1;
        Amet::new(one,two,three,four,five).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Dolor
///
/// - ID: 1028 (0x404)
/// - Size: 8 bytes
/// - Transmitter: Sit
#[derive(Clone, Copy)]
pub struct Dolor {
    raw: [u8; 8],
}

impl Dolor {
    pub const MESSAGE_ID: u32 = 1028;
    
    pub const ONE_FLOAT_MIN: f32 = 0_f32;
    pub const ONE_FLOAT_MAX: f32 = 130_f32;
    
    /// Construct new Dolor from values
    pub fn new(one_float: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_one_float(one_float)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// OneFloat
    ///
    /// - Min: 0
    /// - Max: 130
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn one_float(&self) -> DolorOneFloat {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<u16>();
        
        match signal {
            3 => DolorOneFloat::Dolor,
            5 => DolorOneFloat::Other,
            _ => DolorOneFloat::_Other(self.one_float_raw()),
        }
    }
    
    /// Get raw value of OneFloat
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.5
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn one_float_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<u16>();
        
        let factor = 0.5_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of OneFloat
    #[inline(always)]
    pub fn set_one_float(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 130_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 1028 });
        }
        let factor = 0.5_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor).round() as u16;
        
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Dolor {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Dolor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Dolor")
                .field("one_float", &self.one_float())
            .finish()
        } else {
            f.debug_tuple("Dolor").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Dolor {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let one_float = u.float_in_range(0_f32..=130_f32)?;
        Dolor::new(one_float).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for OneFloat
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DolorOneFloat {
    Dolor,
    Other,
    _Other(f32),
}

impl Into<f32> for DolorOneFloat {
    fn into(self) -> f32 {
        match self {
            DolorOneFloat::Dolor => 3_f32,
            DolorOneFloat::Other => 5_f32,
            DolorOneFloat::_Other(x) => x,
        }
    }
}


/// MultiplexTest
///
/// - ID: 200 (0xc8)
/// - Size: 8 bytes
/// - Transmitter: SENSOR
#[derive(Clone, Copy)]
pub struct MultiplexTest {
    raw: [u8; 8],
}

impl MultiplexTest {
    pub const MESSAGE_ID: u32 = 200;
    
    pub const MULTIPLEXOR_MIN: u8 = 0_u8;
    pub const MULTIPLEXOR_MAX: u8 = 2_u8;
    pub const UNMULTIPLEXED_SIGNAL_MIN: u8 = 0_u8;
    pub const UNMULTIPLEXED_SIGNAL_MAX: u8 = 4_u8;
    pub const MULTIPLEXED_SIGNAL_ZERO_A_MIN: f32 = 0_f32;
    pub const MULTIPLEXED_SIGNAL_ZERO_A_MAX: f32 = 3_f32;
    pub const MULTIPLEXED_SIGNAL_ZERO_B_MIN: f32 = 0_f32;
    pub const MULTIPLEXED_SIGNAL_ZERO_B_MAX: f32 = 3_f32;
    pub const MULTIPLEXED_SIGNAL_ONE_A_MIN: f32 = 0_f32;
    pub const MULTIPLEXED_SIGNAL_ONE_A_MAX: f32 = 6_f32;
    pub const MULTIPLEXED_SIGNAL_ONE_B_MIN: f32 = 0_f32;
    pub const MULTIPLEXED_SIGNAL_ONE_B_MAX: f32 = 6_f32;
    
    /// Construct new MultiplexTest from values
    pub fn new(multiplexor: u8, unmultiplexed_signal: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_multiplexor(multiplexor)?;
        res.set_unmultiplexed_signal(unmultiplexed_signal)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// Get raw value of Multiplexor
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn multiplexor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        signal
    }
    
    pub fn multiplexor(&mut self) -> Result<MultiplexTestMultiplexor, CanError> {
        match self.multiplexor_raw() {
            0 => Ok(MultiplexTestMultiplexor::M0(MultiplexTestMultiplexorM0{ raw: self.raw })),
            1 => Ok(MultiplexTestMultiplexor::M1(MultiplexTestMultiplexorM1{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 200, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Multiplexor
    #[inline(always)]
    fn set_multiplexor(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 2_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 200 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m0(&mut self, value: MultiplexTestMultiplexorM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(0)?;
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m1(&mut self, value: MultiplexTestMultiplexorM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(1)?;
        Ok(())
    }
    
    /// UnmultiplexedSignal
    ///
    /// - Min: 0
    /// - Max: 4
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn unmultiplexed_signal(&self) -> u8 {
        self.unmultiplexed_signal_raw()
    }
    
    /// Get raw value of UnmultiplexedSignal
    ///
    /// - Start bit: 4
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn unmultiplexed_signal_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..12].load_le::<u8>();
        
        signal
    }
    
    /// Set value of UnmultiplexedSignal
    #[inline(always)]
    pub fn set_unmultiplexed_signal(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 4_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 200 });
        }
        self.raw.view_bits_mut::<Lsb0>()[4..12].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MultiplexTest {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for MultiplexTest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("MultiplexTest")
                .field("unmultiplexed_signal", &self.unmultiplexed_signal())
            .finish()
        } else {
            f.debug_tuple("MultiplexTest").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for MultiplexTest {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let multiplexor = u.int_in_range(0..=2)?;
        let unmultiplexed_signal = u.int_in_range(0..=4)?;
        MultiplexTest::new(multiplexor,unmultiplexed_signal).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal MultiplexTest
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum MultiplexTestMultiplexor {
    M0(MultiplexTestMultiplexorM0),
    M1(MultiplexTestMultiplexorM1),
}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct MultiplexTestMultiplexorM0 { raw: [u8; 8] }

impl MultiplexTestMultiplexorM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// MultiplexedSignalZeroA
///
/// - Min: 0
/// - Max: 3
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn multiplexed_signal_zero_a(&self) -> f32 {
    self.multiplexed_signal_zero_a_raw()
}

/// Get raw value of MultiplexedSignalZeroA
///
/// - Start bit: 12
/// - Signal size: 8 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn multiplexed_signal_zero_a_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[12..20].load_le::<u8>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of MultiplexedSignalZeroA
#[inline(always)]
pub fn set_multiplexed_signal_zero_a(&mut self, value: f32) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_f32 || 3_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 200 });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor).round() as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[12..20].store_le(value);
    Ok(())
}

/// MultiplexedSignalZeroB
///
/// - Min: 0
/// - Max: 3
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn multiplexed_signal_zero_b(&self) -> f32 {
    self.multiplexed_signal_zero_b_raw()
}

/// Get raw value of MultiplexedSignalZeroB
///
/// - Start bit: 20
/// - Signal size: 8 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn multiplexed_signal_zero_b_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[20..28].load_le::<u8>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of MultiplexedSignalZeroB
#[inline(always)]
pub fn set_multiplexed_signal_zero_b(&mut self, value: f32) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_f32 || 3_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 200 });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor).round() as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[20..28].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct MultiplexTestMultiplexorM1 { raw: [u8; 8] }

impl MultiplexTestMultiplexorM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// MultiplexedSignalOneA
///
/// - Min: 0
/// - Max: 6
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn multiplexed_signal_one_a(&self) -> f32 {
    self.multiplexed_signal_one_a_raw()
}

/// Get raw value of MultiplexedSignalOneA
///
/// - Start bit: 12
/// - Signal size: 8 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn multiplexed_signal_one_a_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[12..20].load_le::<u8>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of MultiplexedSignalOneA
#[inline(always)]
pub fn set_multiplexed_signal_one_a(&mut self, value: f32) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_f32 || 6_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 200 });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor).round() as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[12..20].store_le(value);
    Ok(())
}

/// MultiplexedSignalOneB
///
/// - Min: 0
/// - Max: 6
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn multiplexed_signal_one_b(&self) -> f32 {
    self.multiplexed_signal_one_b_raw()
}

/// Get raw value of MultiplexedSignalOneB
///
/// - Start bit: 20
/// - Signal size: 8 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn multiplexed_signal_one_b_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[20..28].load_le::<u8>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of MultiplexedSignalOneB
#[inline(always)]
pub fn set_multiplexed_signal_one_b(&mut self, value: f32) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_f32 || 6_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 200 });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor).round() as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[20..28].store_le(value);
    Ok(())
}

}



/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", feature = "std"), derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: u32,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for CanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}

