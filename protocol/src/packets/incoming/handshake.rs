use crate::packets::*;

packet!(Handshake {
    protocol_version VarInt;
}, (p, ctx) -> {
    println!("handling handshake {}", p.protocol_version);

    Ok(())
});
