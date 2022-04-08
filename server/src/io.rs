use std::io::{BufReader, BufWriter, Cursor, Read};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::Mutex;

use anyhow::Result;
use byteorder::ReadBytesExt;
use essentials::app::Context;
use protocol::io::{Readable, VarInt};
use protocol::packets::incoming::ClientHandshakePacket;

use crate::tools::{print_packet, trace};

const MAX_PACKET_SIZE: usize = 32768;

#[derive(PartialEq)]
pub(crate) enum ClientChannelSignal {
    KeepAlive,
    Shutdown,
}

#[derive(PartialEq)]
pub(crate) enum ClientStatus {
    Handshake,
    Status,
    Login,
    Play,
}

pub(crate) struct ClientChannel {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    status: ClientStatus,
}

impl ClientChannel {
    pub(crate) fn new(tcp_stream: TcpStream) -> Result<Self> {
        Ok(Self {
            reader: BufReader::new(tcp_stream.try_clone()?),
            writer: BufWriter::new(tcp_stream),
            status: ClientStatus::Handshake,
        })
    }

    pub(crate) fn accept(&mut self) -> Result<ClientChannelSignal> {
        log::debug!("socket connection accepting from {}", self.addr()?);

        let mut signal = ClientChannelSignal::KeepAlive;

        while signal != ClientChannelSignal::Shutdown {
            match self.receive() {
                Ok(s) => signal = s,
                Err(e) => {
                    self.shutdown()?;

                    return Err(e);
                }
            }
        }

        Ok(signal)
    }

    pub(crate) fn receive(&mut self) -> Result<ClientChannelSignal> {
        let packet = {
            let mut data = [0u8; MAX_PACKET_SIZE];

            let size = self.reader.read(&mut data)?;

            // Close socket signal
            if size == 0 {
                self.shutdown()?;
            }

            data[..size].to_vec()
        };

        if packet.len() == 0 {
            return Ok(ClientChannelSignal::Shutdown);
        }

        if trace() {
            print_packet(&packet);
        }

        self.process_packet(&packet)?;

        Ok(ClientChannelSignal::KeepAlive)
    }

    fn process_packet(&self, data: &[u8]) -> Result<()> {
        let mut cursor = Cursor::new(data);

        match self.status {
            ClientStatus::Handshake => {
                // For some reason we have to ignore first byte
                cursor.read_u8()?;

                let packet = ClientHandshakePacket::read(&mut cursor)?;

                packet.process(&Context)?
            }
            ClientStatus::Status => {}
            ClientStatus::Login => {}
            ClientStatus::Play => {}
        }

        Ok(())
    }

    pub(crate) fn shutdown(&self) -> Result<()> {
        log::debug!("socket client {} disconnected", self.addr()?);

        self.reader.get_ref().shutdown(Shutdown::Read)?;
        self.writer.get_ref().shutdown(Shutdown::Write)?;

        Ok(())
    }

    pub(crate) fn addr(&self) -> Result<SocketAddr> {
        Ok(self.reader.get_ref().peer_addr()?)
    }
}

pub(crate) struct Client {
    pub(crate) id: String,
    pub(crate) channel: Mutex<ClientChannel>,
}

impl Client {
    pub(crate) fn new(tcp_stream: TcpStream) -> Result<Self> {
        Ok(Self {
            id: nuid::next(),
            channel: Mutex::new(ClientChannel::new(tcp_stream)?),
        })
    }
}
