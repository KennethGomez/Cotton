use std::collections::HashMap;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::Arc;
use std::thread;

use anyhow::{anyhow, Result};
use essentials::config::CottonConfiguration;

use crate::io::Client;

pub(crate) mod io;
pub(crate) mod tools;

pub fn initialize(config: &CottonConfiguration) -> Result<MinecraftServer> {
    let mut server = MinecraftServer::new((config.server().ip(), config.server().port()))?;

    log::info!(
        "server binded correctly to {:?}",
        server.tcp_listener.local_addr()
    );

    server.listen()?;

    Ok(server)
}

pub struct MinecraftServer {
    tcp_listener: TcpListener,
    clients: HashMap<String, Arc<Client>>,
}

impl MinecraftServer {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<MinecraftServer> {
        let tcp_listener = TcpListener::bind(addr)?;

        tcp_listener.set_nonblocking(true)?;

        Ok(MinecraftServer {
            tcp_listener,
            clients: HashMap::new(),
        })
    }

    pub fn listen(&mut self) -> Result<()> {
        for s in self.tcp_listener.try_clone()?.incoming() {
            match s {
                Ok(stream) => {
                    let client = Arc::new(Client::new(stream)?);

                    self.add_client(client.clone());

                    handle_client(client.clone());

                    self.drop_client(&client)
                }
                Err(e) => log::error!("error handling TCP connection: `{}`", e),
            }
        }

        Ok(())
    }

    pub(crate) fn add_client(&mut self, client: Arc<Client>) {
        self.clients.insert(client.id.clone(), client);
    }

    pub(crate) fn drop_client(&mut self, client: &Arc<Client>) {
        self.clients.remove(&client.id);
    }
}

fn handle_client(client: Arc<Client>) {
    // TODO: Limit number of threads at a time to a configuration variable
    // https://docs.rs/threadpool/latest/threadpool/
    thread::spawn(move || {
        client
            .channel
            .lock()
            .map_err(|e| anyhow!("error trying to lock client channel mutex: `{}`", e))
            .unwrap()
            .accept()
            .ok();
    });
}
