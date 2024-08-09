pub mod messages {
    // include the output of dbc-codegen placed in the output directory by
    // dbc-codegen.
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

const EXT_FRAME_FLAG: u32 = 0x80000000;

fn main() {
    use messages::Messages;

    // identifier we may receive on a can interface
    let id = 0xCF004FE;

    // id is bit-or'd with EXT_FRAME_FLAG to identify it as an extended id.
    // payload data is only for example.
    let message =
        Messages::from_can_message(id | EXT_FRAME_FLAG, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

    match message {
        Messages::Eec1(eec1) => {
            println!("Got speed: {:?} rpm", eec1.engine_speed());
        }
        _ => panic!("Wrong message!"),
    }
}
