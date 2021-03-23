#![allow(clippy::float_cmp)]

use can_messages::{Amet, Bar, BarThree, CanError, Foo};

#[test]
#[cfg(feature = "range_checked")]
fn check_range_value_error() {
    let result = Bar::new(1, 2.0, 3, 4, true);
    assert!(matches!(
        result,
        Err(CanError::ParameterOutOfRange { message_id: 512 })
    ));
}

#[test]
#[cfg(feature = "range_checked")]
fn check_range_value_valid() {
    let result = Bar::new(1, 2.0, 3, 3, true);
    assert!(result.is_ok());
}

#[test]
fn pack_unpack_message() {
    let result = Foo::new(63.9990234375, 10.0).unwrap();
    assert_eq!(result.voltage_raw(), 63.99899);
    assert_eq!(result.current_raw(), 10.0);
}

#[test]
fn pack_unpack_message_negative() {
    let result = Foo::new(0.000976562, -3.0 * 0.0625).unwrap();
    assert_eq!(result.voltage_raw(), 0.000976562);
    assert_eq!(result.current_raw(), -3.0 * 0.0625);
}

#[test]
fn pack_unpack_message2() {
    let result = Amet::new(1, 0.39, 3, 3, true).unwrap();
    assert_eq!(result.one_raw(), 1);
    assert_eq!(result.two_raw(), 0.39);
    assert_eq!(result.three_raw(), 3);
    assert_eq!(result.four_raw(), 3);
    assert_eq!(result.five_raw(), true);
}

#[test]
#[cfg(feature = "debug")]
fn debug_impl() {
    let result = Bar::new(1, 2.0, 3, 3, true).unwrap();
    let dbg = format!("{:?}", result);
    assert_eq!(&dbg, "Bar([5, 94, 0, 64, 0, 0, 0, 0])");
}

#[test]
#[cfg(feature = "debug")]
fn debug_alternative_impl() {
    let result = Bar::new(1, 2.0, 3, 3, true).unwrap();
    let dbg = format!("{:#?}", result);
    assert_eq!(
        &dbg,
        "Bar {\n    one: 1,\n    two: 1.9499999,\n    three: Onest,\n    four: Onest,\n    xtype: X1on,\n}"
    );
}

#[test]
fn from_raw_to_enum() {
    assert_eq!(BarThree::from(3u8), BarThree::Onest);
}

#[test]
fn from_enum_into_raw() {
    let raw: u8 = BarThree::Onest.into();
    assert_eq!(raw, 3);
}
