//! CAN DBC code generator for Rust
//!
//! DBC files are descriptions of CAN frames.
//! See [this post](https://www.kvaser.com/developer-blog/an-introduction-j1939-and-dbc-files/)
//! for an introduction.

#![deny(missing_docs, clippy::integer_arithmetic)]

use anyhow::{anyhow, ensure, Context, Result};
use can_dbc::{Message, MultiplexIndicator, Signal, ValDescription, ValueDescription, DBC};
use heck::{CamelCase, SnakeCase};
use pad::PadAdapter;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufWriter, Write},
};

mod includes;
mod keywords;
mod pad;

/// Write Rust structs matching DBC input description to `out` buffer
pub fn codegen(dbc_name: &str, dbc_content: &[u8], out: impl Write, debug: bool) -> Result<()> {
    let dbc = can_dbc::DBC::from_slice(dbc_content).map_err(|e| {
        let msg = "Could not parse dbc file";
        if debug {
            anyhow!("{}: {:#?}", msg, e)
        } else {
            anyhow!("{}", msg)
        }
    })?;
    if debug {
        eprintln!("{:#?}", dbc);
    }
    let mut w = BufWriter::new(out);

    writeln!(&mut w, "// Generated code!")?;
    writeln!(
        &mut w,
        "#![allow(unused_comparisons, unreachable_patterns)]"
    )?;
    writeln!(&mut w, "#![allow(clippy::let_and_return, clippy::eq_op)]")?;
    writeln!(
        &mut w,
        "#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]"
    )?;
    writeln!(&mut w, "#![deny(clippy::integer_arithmetic)]")?;
    writeln!(&mut w)?;
    writeln!(&mut w, "//! Message definitions from file `{:?}`", dbc_name)?;
    writeln!(&mut w, "//!")?;
    writeln!(&mut w, "//! - Version: `{:?}`", dbc.version())?;
    writeln!(&mut w)?;
    writeln!(&mut w, "use core::ops::BitOr;")?;
    writeln!(&mut w, "use bitvec::prelude::*;")?;
    writeln!(w, r##"#[cfg(feature = "arb")]"##)?;
    writeln!(&mut w, "use arbitrary::{{Arbitrary, Unstructured}};")?;
    writeln!(&mut w)?;

    render_dbc(&mut w, &dbc).context("could not generate Rust code")?;

    writeln!(&mut w)?;
    writeln!(&mut w, "/// This is just to make testing easier")?;
    writeln!(&mut w, "#[allow(dead_code)]")?;
    writeln!(&mut w, "fn main() {{}}")?;
    writeln!(&mut w)?;
    w.write_all(include_bytes!("./includes/errors.rs"))?;
    w.write_all(include_bytes!("./includes/arbitrary_helpers.rs"))?;
    writeln!(&mut w)?;

    Ok(())
}

fn render_dbc(mut w: impl Write, dbc: &DBC) -> Result<()> {
    render_root_enum(&mut w, dbc)?;

    for msg in dbc.messages() {
        render_message(&mut w, msg, dbc)
            .with_context(|| format!("write message `{}`", msg.message_name()))?;
        writeln!(w)?;
    }

    Ok(())
}

fn render_root_enum(mut w: impl Write, dbc: &DBC) -> Result<()> {
    writeln!(w, "/// All messages")?;
    writeln!(w, "#[derive(Clone)]")?;
    writeln!(w, r##"#[cfg_attr(feature = "debug", derive(Debug))]"##)?;
    writeln!(w, "pub enum Messages {{")?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        for msg in dbc.messages() {
            writeln!(w, "/// {}", msg.message_name())?;
            writeln!(w, "{0}({0}),", type_name(msg.message_name()))?;
        }
    }
    writeln!(&mut w, "}}")?;
    writeln!(&mut w)?;

    writeln!(w, "impl Messages {{")?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        writeln!(&mut w, "/// Read message from CAN frame")?;
        writeln!(w, "#[inline(never)]")?;
        writeln!(
            &mut w,
            "pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {{",
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(&mut w, "use core::convert::TryFrom;")?;
            writeln!(&mut w)?;
            writeln!(&mut w, "let res = match id {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                for msg in dbc.messages() {
                    writeln!(
                        w,
                        "{} => Messages::{1}({1}::try_from(payload)?),",
                        msg.message_id().0,
                        type_name(msg.message_name())
                    )?;
                }
                writeln!(w, r#"n => return Err(CanError::UnknownMessageId(n)),"#)?;
            }
            writeln!(&mut w, "}};")?;
            writeln!(&mut w, "Ok(res)")?;
        }

        writeln!(&mut w, "}}")?;
    }
    writeln!(&mut w, "}}")?;
    writeln!(&mut w)?;

    Ok(())
}

fn render_message(mut w: impl Write, msg: &Message, dbc: &DBC) -> Result<()> {
    writeln!(w, "/// {}", msg.message_name())?;
    writeln!(w, "///")?;
    writeln!(w, "/// - ID: {0} (0x{0:x})", msg.message_id().0)?;
    writeln!(w, "/// - Size: {} bytes", msg.message_size())?;
    if let can_dbc::Transmitter::NodeName(transmitter) = msg.transmitter() {
        writeln!(w, "/// - Transmitter: {}", transmitter)?;
    }
    if let Some(comment) = dbc.message_comment(*msg.message_id()) {
        writeln!(w, "///")?;
        for line in comment.trim().lines() {
            writeln!(w, "/// {}", line)?;
        }
    }
    writeln!(w, "#[derive(Clone, Copy)]")?;
    writeln!(w, "pub struct {} {{", type_name(msg.message_name()))?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        writeln!(w, "raw: [u8; {}],", msg.message_size())?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "impl {} {{", type_name(msg.message_name()))?;
    {
        let mut w = PadAdapter::wrap(&mut w);

        writeln!(
            &mut w,
            "pub const MESSAGE_ID: u32 = {};",
            msg.message_id().0
        )?;
        writeln!(w)?;

        for signal in msg
            .signals()
            .iter()
            .filter(|sig| signal_to_rust_type(&sig) != "bool")
        {
            let typ = signal_to_rust_type(&signal);
            writeln!(
                &mut w,
                "pub const {sig}_MIN: {typ} = {min}_{typ};",
                sig = field_name(signal.name()).to_uppercase(),
                typ = typ,
                min = signal.min,
            )?;

            writeln!(
                &mut w,
                "pub const {sig}_MAX: {typ} = {max}_{typ};",
                sig = field_name(signal.name()).to_uppercase(),
                typ = typ,
                max = signal.max,
            )?;
        }
        writeln!(w)?;

        writeln!(
            &mut w,
            "/// Construct new {} from values",
            msg.message_name()
        )?;
        let args: Vec<String> = msg
            .signals()
            .iter()
            .filter_map(|signal| {
                if *signal.multiplexer_indicator() == MultiplexIndicator::Plain
                    || *signal.multiplexer_indicator() == MultiplexIndicator::Multiplexor
                {
                    Some(format!(
                        "{}: {}",
                        field_name(signal.name()),
                        signal_to_rust_type(&signal)
                    ))
                } else {
                    None
                }
            })
            .collect();
        writeln!(
            &mut w,
            "pub fn new({}) -> Result<Self, CanError> {{",
            args.join(", ")
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(
                &mut w,
                "let mut res = Self {{ raw: [0u8; {}] }};",
                msg.message_size()
            )?;
            for signal in msg.signals().iter() {
                if *signal.multiplexer_indicator() == MultiplexIndicator::Plain {
                    writeln!(&mut w, "res.set_{0}({0})?;", field_name(signal.name()))?;
                }

                if *signal.multiplexer_indicator() == MultiplexIndicator::Multiplexor {
                    writeln!(&mut w, "res.set_{0}({0})?;", field_name(signal.name()))?;
                }
            }
            writeln!(&mut w, "Ok(res)")?;
        }
        writeln!(&mut w, "}}")?;
        writeln!(w)?;

        writeln!(&mut w, "/// Access message payload raw value")?;
        writeln!(&mut w, "pub fn raw(&self) -> &[u8] {{")?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(&mut w, "&self.raw")?;
        }
        writeln!(&mut w, "}}")?;
        writeln!(w)?;

        for signal in msg.signals().iter() {
            match signal.multiplexer_indicator() {
                MultiplexIndicator::Plain => render_signal(&mut w, signal, dbc, msg)
                    .with_context(|| format!("write signal impl `{}`", signal.name()))?,
                MultiplexIndicator::Multiplexor => render_multiplexor_signal(&mut w, signal, msg)?,
                MultiplexIndicator::MultiplexedSignal(_) => {}
                MultiplexIndicator::MultiplexorAndMultiplexedSignal(_) => {}
            }
        }
    }

    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(
        w,
        "impl core::convert::TryFrom<&[u8]> for {} {{",
        type_name(msg.message_name())
    )?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        writeln!(&mut w, "type Error = CanError;")?;
        writeln!(w)?;
        writeln!(w, "#[inline(always)]")?;
        writeln!(
            &mut w,
            "fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {{"
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(
                &mut w,
                r#"if payload.len() != {} {{ return Err(CanError::InvalidPayloadSize); }}"#,
                msg.message_size()
            )?;
            writeln!(&mut w, "let mut raw = [0u8; {}];", msg.message_size())?;
            writeln!(
                &mut w,
                "raw.copy_from_slice(&payload[..{}]);",
                msg.message_size()
            )?;
            writeln!(&mut w, "Ok(Self {{ raw }})")?;
        }
        writeln!(&mut w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    render_debug_impl(&mut w, &msg)?;

    render_arbitrary(&mut w, &msg)?;

    let enums_for_this_message = dbc.value_descriptions().iter().filter_map(|x| {
        if let ValueDescription::Signal {
            message_id,
            signal_name,
            value_descriptions,
        } = x
        {
            if message_id != msg.message_id() {
                return None;
            }
            let signal = dbc.signal_by_name(*message_id, signal_name).unwrap();
            Some((signal, value_descriptions))
        } else {
            None
        }
    });
    for (signal, variants) in enums_for_this_message {
        write_enum(&mut w, signal, msg, variants.as_slice())?;
    }

    let multiplexor_signal = msg
        .signals()
        .iter()
        .find(|s| *s.multiplexer_indicator() == MultiplexIndicator::Multiplexor);

    if let Some(multiplexor_signal) = multiplexor_signal {
        render_multiplexor_enums(w, dbc, msg, multiplexor_signal)?;
    }

    Ok(())
}

fn render_signal(mut w: impl Write, signal: &Signal, dbc: &DBC, msg: &Message) -> Result<()> {
    writeln!(w, "/// {}", signal.name())?;
    if let Some(comment) = dbc.signal_comment(*msg.message_id(), &signal.name()) {
        writeln!(w, "///")?;
        for line in comment.trim().lines() {
            writeln!(w, "/// {}", line)?;
        }
    }
    writeln!(w, "///")?;
    writeln!(w, "/// - Min: {}", signal.min)?;
    writeln!(w, "/// - Max: {}", signal.max)?;
    writeln!(w, "/// - Unit: {:?}", signal.unit())?;
    writeln!(w, "/// - Receivers: {}", signal.receivers().join(", "))?;
    writeln!(w, "#[inline(always)]")?;
    if let Some(variants) = dbc.value_descriptions_for_signal(*msg.message_id(), signal.name()) {
        let type_name = enum_name(msg, signal);

        writeln!(
            w,
            "pub fn {}(&self) -> {} {{",
            field_name(signal.name()),
            type_name,
        )?;
        {
            let match_on_raw_type = match signal_to_rust_type(signal).as_str() {
                "bool" => |x: f64| format!("{}", x),
                // "f32" => |x: f64| format!("x if approx_eq!(f32, x, {}_f32, ulps = 2)", x),
                _ => |x: f64| format!("{}", x),
            };
            let mut w = PadAdapter::wrap(&mut w);
            let read_fn = match signal.byte_order() {
                can_dbc::ByteOrder::LittleEndian => {
                    let (start_bit, end_bit) = le_start_end_bit(signal, msg)?;

                    format!(
                        "self.raw.view_bits::<Lsb0>()[{start}..{end}].load_le::<{typ}>()",
                        typ = signal_to_rust_uint(signal),
                        start = start_bit,
                        end = end_bit,
                    )
                }
                can_dbc::ByteOrder::BigEndian => {
                    let (start_bit, end_bit) = be_start_end_bit(signal, msg)?;

                    format!(
                        "self.raw.view_bits::<Msb0>()[{start}..{end}].load_be::<{typ}>()",
                        typ = signal_to_rust_uint(signal),
                        start = start_bit,
                        end = end_bit
                    )
                }
            };

            writeln!(&mut w, r#"let signal = {};"#, read_fn)?;
            writeln!(&mut w)?;
            writeln!(&mut w, "match signal {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                for variant in variants {
                    let literal = match_on_raw_type(*variant.a());
                    writeln!(
                        &mut w,
                        "{} => {}::{},",
                        literal,
                        type_name,
                        enum_variant_name(variant.b())
                    )?;
                }
                writeln!(
                    &mut w,
                    "_ => {}::_Other(self.{}_raw()),",
                    type_name,
                    field_name(signal.name())
                )?;
            }
            writeln!(&mut w, "}}")?;
        }
        writeln!(&mut w, "}}")?;
        writeln!(w)?;
    } else {
        writeln!(
            w,
            "pub fn {}(&self) -> {} {{",
            field_name(signal.name()),
            signal_to_rust_type(signal)
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(&mut w, "self.{}_raw()", field_name(signal.name()))?;
        }
        writeln!(&mut w, "}}")?;
        writeln!(w)?;
    }

    writeln!(w, "/// Get raw value of {}", signal.name())?;
    writeln!(w, "///")?;
    writeln!(w, "/// - Start bit: {}", signal.start_bit)?;
    writeln!(w, "/// - Signal size: {} bits", signal.signal_size)?;
    writeln!(w, "/// - Factor: {}", signal.factor)?;
    writeln!(w, "/// - Offset: {}", signal.offset)?;
    writeln!(w, "/// - Byte order: {:?}", signal.byte_order())?;
    writeln!(w, "/// - Value type: {:?}", signal.value_type())?;
    writeln!(w, "#[inline(always)]")?;
    writeln!(
        w,
        "pub fn {}_raw(&self) -> {} {{",
        field_name(signal.name()),
        signal_to_rust_type(&signal)
    )?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        signal_from_payload(&mut w, signal, msg).context("signal from payload")?;
    }
    writeln!(&mut w, "}}")?;
    writeln!(w)?;

    render_set_signal(&mut w, signal, msg)?;

    Ok(())
}

fn render_set_signal(mut w: impl Write, signal: &Signal, msg: &Message) -> Result<()> {
    writeln!(&mut w, "/// Set value of {}", signal.name())?;
    writeln!(w, "#[inline(always)]")?;

    // To avoid accidentially changing the multiplexor value without changing
    // the signals accordingly this fn is kept private for multiplexors.
    let visibility = if *signal.multiplexer_indicator() == MultiplexIndicator::Multiplexor {
        ""
    } else {
        "pub "
    };

    writeln!(
        w,
        "{}fn set_{}(&mut self, value: {}) -> Result<(), CanError> {{",
        visibility,
        field_name(signal.name()),
        signal_to_rust_type(&signal)
    )?;

    {
        let mut w = PadAdapter::wrap(&mut w);

        if signal.signal_size != 1 {
            writeln!(w, r##"#[cfg(feature = "range_checked")]"##)?;
            writeln!(
                w,
                r##"if value < {min}_{typ} || {max}_{typ} < value {{"##,
                typ = signal_to_rust_type(&signal),
                min = signal.min(),
                max = signal.max(),
            )?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                writeln!(
                    w,
                    r##"return Err(CanError::ParameterOutOfRange {{ message_id: {message_id} }});"##,
                    message_id = msg.message_id().0,
                )?;
            }
            writeln!(w, r"}}")?;
        }
        signal_to_payload(&mut w, signal, msg).context("signal to payload")?;
    }

    writeln!(&mut w, "}}")?;
    writeln!(w)?;

    Ok(())
}

fn render_set_signal_multiplexer(
    mut w: impl Write,
    multiplexor: &Signal,
    msg: &Message,
    switch_index: u64,
) -> Result<()> {
    writeln!(&mut w, "/// Set value of {}", multiplexor.name())?;
    writeln!(w, "#[inline(always)]")?;
    writeln!(
        w,
        "pub fn set_{enum_variant_wrapper}(&mut self, value: {enum_variant}) -> Result<(), CanError> {{",
        enum_variant_wrapper = multiplexed_enum_variant_wrapper_name(switch_index).to_snake_case(),
        enum_variant = multiplexed_enum_variant_name(msg, multiplexor, switch_index)?,
    )?;

    {
        let mut w = PadAdapter::wrap(&mut w);

        writeln!(&mut w, "let b0 = BitArray::<LocalBits, _>::new(self.raw);")?;
        writeln!(&mut w, "let b1 = BitArray::<LocalBits, _>::new(value.raw);")?;
        writeln!(&mut w, "self.raw = b0.bitor(b1).into_inner();")?;
        writeln!(
            &mut w,
            "self.set_{}({})?;",
            field_name(multiplexor.name()),
            switch_index
        )?;
        writeln!(&mut w, "Ok(())",)?;
    }

    writeln!(&mut w, "}}")?;
    writeln!(w)?;

    Ok(())
}

fn render_multiplexor_signal(mut w: impl Write, signal: &Signal, msg: &Message) -> Result<()> {
    writeln!(w, "/// Get raw value of {}", signal.name())?;
    writeln!(w, "///")?;
    writeln!(w, "/// - Start bit: {}", signal.start_bit)?;
    writeln!(w, "/// - Signal size: {} bits", signal.signal_size)?;
    writeln!(w, "/// - Factor: {}", signal.factor)?;
    writeln!(w, "/// - Offset: {}", signal.offset)?;
    writeln!(w, "/// - Byte order: {:?}", signal.byte_order())?;
    writeln!(w, "/// - Value type: {:?}", signal.value_type())?;
    writeln!(w, "#[inline(always)]")?;
    writeln!(
        w,
        "pub fn {}_raw(&self) -> {} {{",
        field_name(signal.name()),
        signal_to_rust_type(&signal)
    )?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        signal_from_payload(&mut w, signal, msg).context("signal from payload")?;
    }
    writeln!(&mut w, "}}")?;
    writeln!(w)?;

    writeln!(
        w,
        "pub fn {}(&mut self) -> Result<{}, CanError> {{",
        field_name(signal.name()),
        multiplex_enum_name(msg, signal)?
    )?;

    let multiplexer_indexes: BTreeSet<u64> = msg
        .signals()
        .iter()
        .filter_map(|s| {
            if let MultiplexIndicator::MultiplexedSignal(index) = s.multiplexer_indicator() {
                Some(index)
            } else {
                None
            }
        })
        .cloned()
        .collect();

    {
        let mut w = PadAdapter::wrap(&mut w);
        writeln!(&mut w, "match self.{}_raw() {{", field_name(signal.name()))?;

        {
            let mut w = PadAdapter::wrap(&mut w);
            for multiplexer_index in multiplexer_indexes.iter() {
                writeln!(
                    &mut w,
                    "{idx} => Ok({enum_name}::{multiplexed_wrapper_name}({multiplexed_name}{{ raw: self.raw }})),",
                    idx = multiplexer_index,
                    enum_name = multiplex_enum_name(msg, signal)?,
                    multiplexed_wrapper_name = multiplexed_enum_variant_wrapper_name(*multiplexer_index),
                    multiplexed_name =
                        multiplexed_enum_variant_name(msg, signal, *multiplexer_index)?
                )?;
            }
            writeln!(
                &mut w,
                "multiplexor => Err(CanError::InvalidMultiplexor {{ message_id: {}, multiplexor: multiplexor.into() }}),",
                msg.message_id().0
            )?;
        }

        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;

    render_set_signal(&mut w, signal, msg)?;

    let mut multiplexed_signals = BTreeMap::new();
    for signal in msg.signals() {
        if let MultiplexIndicator::MultiplexedSignal(switch_index) = signal.multiplexer_indicator()
        {
            multiplexed_signals
                .entry(switch_index)
                .and_modify(|v: &mut Vec<&Signal>| v.push(&signal))
                .or_insert_with(|| vec![&signal]);
        }
    }

    for switch_index in multiplexer_indexes {
        render_set_signal_multiplexer(&mut w, signal, msg, switch_index)?;
    }

    Ok(())
}

fn be_start_end_bit(signal: &Signal, msg: &Message) -> Result<(u64, u64)> {
    let err = "calculating start bit";

    let x = signal.start_bit.checked_div(8).context(err)?;
    let x = x.checked_mul(8).context(err)?;

    let y = signal.start_bit.checked_rem(8).context(err)?;
    let y = 7u64.checked_sub(y).context(err)?;

    let start_bit = x.checked_add(y).context(err)?;
    let end_bit = start_bit
        .checked_add(signal.signal_size)
        .context("calculating last bit position")?;

    let msg_bits = msg.message_size().checked_mul(8).unwrap();

    ensure!(
        start_bit <= msg_bits,
        "signal starts at {}, but message is only {} bits",
        start_bit,
        msg_bits
    );
    ensure!(
        end_bit <= msg_bits,
        "signal ends at {}, but message is only {} bits",
        end_bit,
        msg_bits
    );
    Ok((start_bit, end_bit))
}

fn le_start_end_bit(signal: &Signal, msg: &Message) -> Result<(u64, u64)> {
    let msg_bits = msg.message_size().checked_mul(8).unwrap();
    let start_bit = signal.start_bit;
    ensure!(
        start_bit <= msg_bits,
        "signal starts at {}, but message is only {} bits",
        start_bit,
        msg_bits
    );

    let end_bit = signal
        .start_bit
        .checked_add(signal.signal_size)
        .context("overflow calculating last bit position")?;
    ensure!(
        end_bit <= msg_bits,
        "signal ends at {}, but message is only {} bits",
        end_bit,
        msg_bits
    );
    Ok((start_bit, end_bit))
}

fn signal_from_payload(mut w: impl Write, signal: &Signal, msg: &Message) -> Result<()> {
    let read_fn = match signal.byte_order() {
        can_dbc::ByteOrder::LittleEndian => {
            let (start_bit, end_bit) = le_start_end_bit(signal, msg)?;

            format!(
                "self.raw.view_bits::<Lsb0>()[{start}..{end}].load_le::<{typ}>()",
                typ = signal_to_rust_uint(signal),
                start = start_bit,
                end = end_bit,
            )
        }
        can_dbc::ByteOrder::BigEndian => {
            let (start_bit, end_bit) = be_start_end_bit(signal, msg)?;

            format!(
                "self.raw.view_bits::<Msb0>()[{start}..{end}].load_be::<{typ}>()",
                typ = signal_to_rust_uint(signal),
                start = start_bit,
                end = end_bit
            )
        }
    };

    writeln!(&mut w, r#"let signal = {};"#, read_fn)?;
    writeln!(&mut w)?;

    if *signal.value_type() == can_dbc::ValueType::Signed {
        writeln!(
            &mut w,
            "let signal  = {}::from_ne_bytes(signal.to_ne_bytes());",
            signal_to_rust_int(signal)
        )?;
    };

    if signal.signal_size == 1 {
        writeln!(&mut w, "signal == 1")?;
    } else if signal_is_float_in_rust(signal) {
        // Scaling is always done on floats
        writeln!(&mut w, "let factor = {}_f32;", signal.factor)?;
        writeln!(&mut w, "let offset = {}_f32;", signal.offset)?;
        writeln!(&mut w, "(signal as f32) * factor + offset")?;
    } else {
        writeln!(&mut w, "signal")?;
    }
    Ok(())
}

fn signal_to_payload(mut w: impl Write, signal: &Signal, msg: &Message) -> Result<()> {
    if signal.signal_size == 1 {
        // Map boolean to byte so we can pack it
        writeln!(&mut w, "let value = value as u8;")?;
    } else if signal_is_float_in_rust(signal) {
        // Massage value into an int
        writeln!(&mut w, "let factor = {}_f32;", signal.factor)?;
        writeln!(&mut w, "let offset = {}_f32;", signal.offset)?;
        writeln!(
            &mut w,
            "let value = ((value - offset) / factor) as {};",
            signal_to_rust_int(signal)
        )?;
        writeln!(&mut w)?;
    }

    if *signal.value_type() == can_dbc::ValueType::Signed {
        writeln!(
            &mut w,
            "let value = {}::from_ne_bytes(value.to_ne_bytes());",
            signal_to_rust_uint(signal)
        )?;
    };

    match signal.byte_order() {
        can_dbc::ByteOrder::LittleEndian => {
            let (start_bit, end_bit) = le_start_end_bit(signal, msg)?;
            writeln!(
                &mut w,
                r#"self.raw.view_bits_mut::<Lsb0>()[{start_bit}..{end_bit}].store_le(value);"#,
                start_bit = start_bit,
                end_bit = end_bit,
            )?;
        }
        can_dbc::ByteOrder::BigEndian => {
            let (start_bit, end_bit) = be_start_end_bit(signal, msg)?;
            writeln!(
                &mut w,
                r#"self.raw.view_bits_mut::<Msb0>()[{start_bit}..{end_bit}].store_be(value);"#,
                start_bit = start_bit,
                end_bit = end_bit,
            )?;
        }
    };

    writeln!(&mut w, "Ok(())")?;
    Ok(())
}

fn write_enum(
    mut w: impl Write,
    signal: &Signal,
    msg: &Message,
    variants: &[ValDescription],
) -> Result<()> {
    let type_name = enum_name(msg, signal);
    let signal_rust_type = signal_to_rust_type(signal);

    writeln!(w, "/// Defined values for {}", signal.name())?;
    writeln!(w, "#[derive(Clone, Copy, PartialEq)]")?;
    writeln!(w, r##"#[cfg_attr(feature = "debug", derive(Debug))]"##)?;
    writeln!(w, "pub enum {} {{", type_name)?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        for variant in variants {
            writeln!(w, "{},", enum_variant_name(variant.b()))?;
        }
        writeln!(w, "_Other({}),", signal_rust_type)?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "impl Into<{}> for {} {{", signal_rust_type, type_name)?;
    {
        let match_on_raw_type = match signal_to_rust_type(signal).as_str() {
            "bool" => |x: f64| format!("{}", (x as i64) == 1),
            "f32" => |x: f64| format!("{}_f32", x),
            _ => |x: f64| format!("{}", x as i64),
        };

        let mut w = PadAdapter::wrap(&mut w);
        writeln!(w, "fn into(self) -> {} {{", signal_rust_type)?;
        {
            let mut w = PadAdapter::wrap(&mut w);

            writeln!(&mut w, "match self {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                for variant in variants {
                    let literal = match_on_raw_type(*variant.a());
                    writeln!(
                        &mut w,
                        "{}::{} => {},",
                        type_name,
                        enum_variant_name(variant.b()),
                        literal,
                    )?;
                }
                writeln!(&mut w, "{}::_Other(x) => x,", type_name,)?;
            }
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    Ok(())
}

fn signal_to_rust_int(signal: &Signal) -> String {
    let sign = match signal.value_type() {
        can_dbc::ValueType::Signed => "i",
        can_dbc::ValueType::Unsigned => "u",
    };

    let size = match *signal.signal_size() {
        n if n <= 8 => "8",
        n if n <= 16 => "16",
        n if n <= 32 => "32",
        _ => "64",
    };

    format!("{}{}", sign, size)
}

fn signal_to_rust_uint(signal: &Signal) -> String {
    let size = match *signal.signal_size() {
        n if n <= 8 => "8",
        n if n <= 16 => "16",
        n if n <= 32 => "32",
        _ => "64",
    };

    format!("u{}", size)
}

#[allow(clippy::float_cmp)]
fn signal_is_float_in_rust(signal: &Signal) -> bool {
    *signal.offset() != 0.0 || *signal.factor() != 1.0
}

fn signal_to_rust_type(signal: &Signal) -> String {
    if signal.signal_size == 1 {
        String::from("bool")
    } else if signal_is_float_in_rust(signal) {
        // If there is any scaling needed, go for float
        String::from("f32")
    } else {
        signal_to_rust_int(signal)
    }
}

fn type_name(x: &str) -> String {
    if keywords::is_keyword(x) || !x.starts_with(|c: char| c.is_ascii_alphabetic()) {
        format!("X{}", x.to_camel_case())
    } else {
        x.to_camel_case()
    }
}

fn field_name(x: &str) -> String {
    if keywords::is_keyword(x) || !x.starts_with(|c: char| c.is_ascii_alphabetic()) {
        format!("x{}", x.to_snake_case())
    } else {
        x.to_snake_case()
    }
}

fn enum_name(msg: &Message, signal: &Signal) -> String {
    // this turns signal `_4DRIVE` into `4drive`
    let signal_name = signal
        .name()
        .trim_start_matches(|c: char| c.is_ascii_punctuation());

    format!(
        "{}{}",
        enum_variant_name(msg.message_name()),
        signal_name.to_camel_case()
    )
}

fn enum_variant_name(x: &str) -> String {
    if keywords::is_keyword(x) || !x.starts_with(|c: char| c.is_ascii_alphabetic()) {
        format!("X{}", x.to_camel_case())
    } else {
        x.to_camel_case()
    }
}

fn multiplexed_enum_variant_wrapper_name(switch_index: u64) -> String {
    format!("M{}", switch_index)
}

fn multiplex_enum_name(msg: &Message, multiplexor: &Signal) -> Result<String> {
    ensure!(
        matches!(
            multiplexor.multiplexer_indicator(),
            MultiplexIndicator::Multiplexor
        ),
        "signal {:?} is not the multiplexor",
        multiplexor
    );
    Ok(format!(
        "{}{}",
        msg.message_name().to_camel_case(),
        multiplexor.name().to_camel_case()
    ))
}

fn multiplexed_enum_variant_name(
    msg: &Message,
    multiplexor: &Signal,
    switch_index: u64,
) -> Result<String> {
    ensure!(
        matches!(
            multiplexor.multiplexer_indicator(),
            MultiplexIndicator::Multiplexor
        ),
        "signal {:?} is not the multiplexor",
        multiplexor
    );

    Ok(format!(
        "{}{}M{}",
        msg.message_name().to_camel_case(),
        multiplexor.name().to_camel_case(),
        switch_index
    ))
}

fn render_debug_impl(mut w: impl Write, msg: &Message) -> Result<()> {
    let typ = type_name(msg.message_name());
    writeln!(w, r##"#[cfg(feature = "debug")]"##)?;
    writeln!(w, r##"impl core::fmt::Debug for {} {{"##, typ)?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        writeln!(
            w,
            "fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(w, r#"if f.alternate() {{"#)?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                writeln!(w, r#"f.debug_struct("{}")"#, typ)?;
                {
                    let mut w = PadAdapter::wrap(&mut w);
                    for signal in msg.signals() {
                        if *signal.multiplexer_indicator() == MultiplexIndicator::Plain {
                            writeln!(
                                w,
                                r#".field("{field_name}", &self.{field_name}())"#,
                                field_name = field_name(signal.name()),
                            )?;
                        }
                    }
                }
                writeln!(w, r#".finish()"#)?;
            }
            writeln!(w, r#"}} else {{"#)?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                writeln!(w, r#"f.debug_tuple("{}").field(&self.raw).finish()"#, typ)?;
            }
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    Ok(())
}

fn render_multiplexor_enums(
    mut w: impl Write,
    dbc: &DBC,
    msg: &Message,
    multiplexor_signal: &Signal,
) -> Result<()> {
    ensure!(
        *multiplexor_signal.multiplexer_indicator() == MultiplexIndicator::Multiplexor,
        "signal {} is not the multiplexor",
        multiplexor_signal.name(),
    );

    let mut multiplexed_signals = BTreeMap::new();
    for signal in msg.signals() {
        if let MultiplexIndicator::MultiplexedSignal(switch_index) = signal.multiplexer_indicator()
        {
            multiplexed_signals
                .entry(switch_index)
                .and_modify(|v: &mut Vec<&Signal>| v.push(&signal))
                .or_insert_with(|| vec![&signal]);
        }
    }

    writeln!(
        w,
        "/// Defined values for multiplexed signal {}",
        msg.message_name()
    )?;
    writeln!(w, r##"#[cfg_attr(feature = "debug", derive(Debug))]"##)?;

    writeln!(
        w,
        "pub enum {} {{",
        multiplex_enum_name(msg, multiplexor_signal)?
    )?;

    {
        let mut w = PadAdapter::wrap(&mut w);
        for (switch_index, _multiplexed_signals) in multiplexed_signals.iter() {
            writeln!(
                w,
                "{multiplexed_wrapper_name}({multiplexed_name}),",
                multiplexed_wrapper_name = multiplexed_enum_variant_wrapper_name(**switch_index),
                multiplexed_name =
                    multiplexed_enum_variant_name(msg, multiplexor_signal, **switch_index)?
            )?;
        }
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    for (switch_index, multiplexed_signals) in multiplexed_signals.iter() {
        writeln!(w, r##"#[derive(Default)]"##)?;
        writeln!(w, r##"#[cfg_attr(feature = "debug", derive(Debug))]"##)?;
        let struct_name = multiplexed_enum_variant_name(msg, multiplexor_signal, **switch_index)?;
        writeln!(
            w,
            "pub struct {} {{ raw: [u8; {}] }}",
            struct_name,
            msg.message_size()
        )?;
        writeln!(w)?;

        writeln!(w, "impl {} {{", struct_name)?;

        writeln!(
            w,
            "pub fn new() -> Self {{ Self {{ raw: [0u8; {}] }} }}",
            msg.message_size()
        )?;

        for signal in multiplexed_signals {
            render_signal(&mut w, signal, dbc, msg)?;
        }

        writeln!(w, "}}")?;
        writeln!(w)?;
    }

    Ok(())
}

fn render_arbitrary(mut w: impl Write, msg: &Message) -> Result<()> {
    writeln!(w, r##"#[cfg(feature = "arb")]"##)?;
    writeln!(
        w,
        "impl<'a> Arbitrary<'a> for {typ} {{",
        typ = type_name(msg.message_name())
    )?;
    {
        let mut w = PadAdapter::wrap(&mut w);
        writeln!(
            w,
            "fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {{"
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            let filtered_signals: Vec<&Signal> = msg
                .signals()
                .iter()
                .filter(|signal| {
                    *signal.multiplexer_indicator() == MultiplexIndicator::Plain
                        || *signal.multiplexer_indicator() == MultiplexIndicator::Multiplexor
                })
                .collect();

            for signal in filtered_signals.iter() {
                writeln!(
                    w,
                    "let {field_name} = {arbitrary_value};",
                    field_name = field_name(signal.name()),
                    arbitrary_value = signal_to_arbitrary(signal),
                )?;
            }

            let args: Vec<String> = filtered_signals
                .iter()
                .map(|signal| field_name(signal.name()))
                .collect();

            writeln!(
                w,
                "{typ}::new({args}).map_err(|_| arbitrary::Error::IncorrectFormat)",
                typ = type_name(msg.message_name()),
                args = args.join(",")
            )?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;

    Ok(())
}

fn signal_to_arbitrary(signal: &Signal) -> String {
    if signal.signal_size == 1 {
        "u.int_in_range(0..=1)? == 1".to_string()
    } else if signal_is_float_in_rust(signal) {
        format!(
            "u.float_in_range({min}_f32..={max}_f32)?",
            min = signal.min(),
            max = signal.max()
        )
    } else {
        format!(
            "u.int_in_range({min}..={max})?",
            min = signal.min(),
            max = signal.max()
        )
    }
}
