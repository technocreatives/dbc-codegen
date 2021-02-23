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
