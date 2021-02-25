mod messages;
pub use messages::*;

#[test]
#[cfg(feature = "range_checked")]
fn check_range_value_error() {
    let result = messages::Bar::new(1, 2.0, 3, 4);
    assert!(matches!(
        result,
        Err(CanError::ParameterOutOfRange { message_id: 512 })
    ));
}

#[test]
#[cfg(feature = "range_checked")]
fn check_range_value_valid() {
    let result = messages::Bar::new(1, 2.0, 3, 3);
    assert!(result.is_ok());
}

#[test]
fn pack_unpack_message() {
    let result = messages::Foo::new(63.99899, 10.0).unwrap();
    assert_eq!(result.voltage_raw(), 63.99899);
    assert_eq!(result.current_raw(), 10.0);
}

#[test]
fn pack_unpack_message2() {
    let result = messages::Bar::new(1, 0.39, 3, 3).unwrap();
    assert_eq!(result.one_raw(), 1);
    assert_eq!(result.two_raw(), 0.39);
    assert_eq!(result.three_raw(), 3);
    assert_eq!(result.four_raw(), 3);
}
