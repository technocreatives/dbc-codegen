mod messages;
pub use messages::*;

#[test]
#[cfg(feature = "range_checked")]
fn change_range_value() {
    let bar = messages::Bar::new(1, 2.0, 3, 3);
}
