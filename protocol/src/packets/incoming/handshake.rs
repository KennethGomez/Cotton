use crate::packets::*;

packet!(Handshake {
    protocol_version: VarInt;
    server_address: String;
    server_port: u16;
    next_state: VarInt;
}, (p, ctx) -> {
    println!("handling handshake {}", p.protocol_version);

    Ok(())
});
