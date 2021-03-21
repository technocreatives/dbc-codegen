#![no_main]
use libfuzzer_sys::arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use cantools_messages::{
    example_bar_four_encode, example_bar_one_encode, example_bar_pack, example_bar_t,
    example_bar_three_encode, example_bar_two_encode, example_bar_type_encode,
};

fuzz_target!(|dbc_codegen_bar: can_messages::Bar| {
    let dbc_codegen_bar = can_messages::Bar::new(3, 2.0, 4, 5, false).unwrap();

    println!(
        "{} {} {} {} {}",
        dbc_codegen_bar.one_raw(),
        dbc_codegen_bar.two_raw(),
        dbc_codegen_bar.three_raw(),
        dbc_codegen_bar.four_raw(),
        dbc_codegen_bar.xtype_raw()
    );

    let one = unsafe { example_bar_one_encode(dbc_codegen_bar.one_raw() as f64) };
    let two = unsafe { example_bar_two_encode(dbc_codegen_bar.two_raw() as f64) };
    let three = unsafe { example_bar_three_encode(dbc_codegen_bar.three_raw() as f64) };
    let four = unsafe { example_bar_four_encode(dbc_codegen_bar.four_raw() as f64) };
    let type_ = unsafe { example_bar_type_encode(dbc_codegen_bar.xtype_raw() as u8 as f64) };

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
});
