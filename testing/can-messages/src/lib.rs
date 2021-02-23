mod messages;
pub use messages::*;

#[test]
#[cfg(feature = "range_checked")]
fn check_range_value_error() {
    let result = messages::Bar::new(1, 2.0, 3, 4);
    assert!(matches!(
        result,
        Err(CanError::ParameterOutOfRange { min: 0.0, max: 3.0 })
    ));
}

#[test]
#[cfg(feature = "range_checked")]
fn check_range_value_valid() {
    let result = messages::Bar::new(1, 2.0, 3, 3);
    assert!(result.is_ok());
}
