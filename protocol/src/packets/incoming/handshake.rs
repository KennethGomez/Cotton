use crate::packets::*;

packet!(Handshake {
    protocol_version VarInt;
}, ctx -> {
    println!("handling handshake");

    Ok(())
});
