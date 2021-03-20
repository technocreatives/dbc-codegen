use can_messages::Messages;

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("first cli arg should be candump file (`candump vcan0 -l`)");
    let file = std::fs::read_to_string(&input).unwrap();
    for line in file.lines() {
        let data = line.split(' ').last().unwrap();
        let (id, payload) = {
            let mut s = data.split('#');
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
        };

        match Messages::from_can_message(id, &payload) {
            Ok(Messages::Foo(msg)) => {
                dbg!(msg.voltage());
                dbg!(msg.current());
            }
            Ok(Messages::Bar(msg)) => {
                dbg!(msg.one());
                dbg!(msg.two());
                dbg!(msg.three());
                dbg!(msg.four());
            }
            msg => {
                let _ = dbg!(msg);
            }
        }
    }

    // // just for fun, let's construct a message as well!
    // let msg = can_messages::Foo::new(16.1, 16.2);
    // dbg!(msg);
}
