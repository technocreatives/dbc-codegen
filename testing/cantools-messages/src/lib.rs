#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
#[ignore]
fn pack_message() {
    let dbc_codegen_bar = can_messages::Bar::new(1, 0.0, 0, 0, false).unwrap();
    let one = unsafe { example_bar_one_encode(1.0) };
    let two = unsafe { example_bar_two_encode(0.0) };
    let three = unsafe { example_bar_three_encode(0.0) };
    let four = unsafe { example_bar_four_encode(0.0) };
    let five = unsafe { example_bar_five_encode(0.0) };

    let bar = example_bar_t {
        one,
        two,
        three,
        four,
        five,
    };
    let mut buffer: [u8; 8] = [0; 8];
    unsafe { example_bar_pack(buffer.as_mut_ptr(), &bar, buffer.len() as u64) };
    assert_eq!(dbc_codegen_bar.raw(), buffer);
}

#[test]
#[ignore]
fn pack_message_signed() {
    let dbc_codegen_foo = can_messages::Foo::new(63.99899, -10.0).unwrap();
    let current = unsafe { example_foo_current_encode(-10.0) };
    let voltage = unsafe { example_foo_voltage_encode(63.99899) };

    let foo = example_foo_t { current, voltage };
    let mut buffer: [u8; 4] = [0; 4];
    unsafe { example_foo_pack(buffer.as_mut_ptr(), &foo, buffer.len() as u64) };
    assert_eq!(dbc_codegen_foo.raw(), buffer);
}
