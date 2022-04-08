pub mod handshake;

use super::*;

use handshake::*;

packet_enum!(ClientHandshakePacket {
    0x00 = Handshake,
});
