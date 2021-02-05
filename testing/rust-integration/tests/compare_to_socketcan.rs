use can_messages as messages;
use std::convert::TryFrom;

#[test]
fn simple_byte_aligned_message() {
    let (_, payload) = parse_canframe("100#20000008");
    let msg = messages::Foo::try_from(payload.as_slice()).unwrap();

    assert_f32_eq(msg.voltage(), 2_f32);
    assert_f32_eq(msg.current(), 2_f32);
}

#[test]
fn another_simple_byte_aligned_message() {
    let (_, payload) = parse_canframe("100#D500AEA9");
    let msg = messages::Foo::try_from(payload.as_slice()).unwrap();

    assert_f32_eq(msg.voltage(), 42.42_f32);
    assert_f32_eq(msg.current(), 13.37_f32);
}

#[test]
fn weirdly_aligned_bigendian_message_1() {
    let (_, payload) = parse_canframe("200#0594000000000000");
    let msg = messages::Bar::try_from(payload.as_slice()).unwrap();

    assert_eq!(msg.one(), 2);
    assert_f32_eq(msg.two(), 2_f32);
    assert!(matches!(msg.three(), messages::BarThree::Oner));
    assert!(matches!(msg.four(), messages::BarFour::Oner));
}

#[test]
fn weirdly_aligned_bigendian_message_2() {
    let (_, payload) = parse_canframe("200#055A000000000000");
    let msg = messages::Bar::try_from(payload.as_slice()).unwrap();

    assert_eq!(msg.one(), 1);
    assert_f32_eq(msg.two(), 2_f32);
    assert!(matches!(msg.three(), messages::BarThree::Onest));
    assert!(matches!(msg.four(), messages::BarFour::On));
}

fn parse_canframe(candump_line: &str) -> (u32, Vec<u8>) {
    let mut s = candump_line.split('#');
    let id = s.next().unwrap();
    let id: u32 = u32::from_str_radix(id, 16).unwrap();
    let payload = s.next().unwrap();
    let payload = (0..(payload.len() / 2))
        .map(|i| {
            let pos = i * 2;
            u8::from_str_radix(&payload[pos..pos + 2], 16).unwrap()
        })
        .collect::<Vec<_>>();
    (id, payload)
}

fn assert_f32_eq(a: f32, b: f32) {
    let _ = approx::abs_diff_eq!(a, b);
}
