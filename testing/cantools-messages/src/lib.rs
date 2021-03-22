#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn pack_message() {
    let dbc_codegen_bar = can_messages::Bar::new(3, 2.0, 4, 2, false).unwrap();
    let one = unsafe { example_bar_one_encode(3.0) };
    let two = unsafe { example_bar_two_encode(2.0) };
    let three = unsafe { example_bar_three_encode(4.0) };
    let four = unsafe { example_bar_four_encode(2.0) };
    let type_ = unsafe { example_bar_type_encode(0.0) };

    let bar = example_bar_t {
        one,
        two,
        three,
        four,
        type_,
    };
    let mut buffer: [u8; 8] = [0; 8];
    unsafe { example_bar_pack(buffer.as_mut_ptr(), &bar, buffer.len() as u64) };
    assert_eq!(dbc_codegen_bar.raw(), buffer);
}

#[test]
fn pack_message_signed_negative() {
    let dbc_codegen_foo = can_messages::Foo::new(0.000976562, -3.0 * 0.0625).unwrap();
    let current = unsafe { example_foo_current_encode(-3.0 * 0.0625) };
    let voltage = unsafe { example_foo_voltage_encode(0.000976562) };

    let foo = example_foo_t { current, voltage };
    let mut buffer: [u8; 4] = [0; 4];
    unsafe { example_foo_pack(buffer.as_mut_ptr(), &foo, buffer.len() as u64) };
    assert_eq!(dbc_codegen_foo.raw(), buffer);
}

#[test]
fn pack_message_signed_positive() {
    let dbc_codegen_foo = can_messages::Foo::new(0.000976562, 0.0625).unwrap();
    let current = unsafe { example_foo_current_encode(0.0625) };
    let voltage = unsafe { example_foo_voltage_encode(0.000976562) };

    let foo = example_foo_t { current, voltage };
    let mut buffer: [u8; 4] = [0; 4];
    unsafe { example_foo_pack(buffer.as_mut_ptr(), &foo, buffer.len() as u64) };
    assert_eq!(dbc_codegen_foo.raw(), buffer);
}

#[test]
fn pack_big_endian_signal_with_start_bit_zero() {
    let dbc_codegen_bar = can_messages::Dolor::new(1).unwrap();
    let one_float = unsafe { example_dolor_one_float_encode(1.0) };

    let dolor = example_dolor_t { one_float };
    let mut buffer: [u8; 8] = [0; 8];
    unsafe { example_dolor_pack(buffer.as_mut_ptr(), &dolor, buffer.len() as u64) };
    assert_eq!(dbc_codegen_bar.raw(), buffer);
}
