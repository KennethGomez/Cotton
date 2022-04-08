pub(crate) fn print_packet(data: &[u8]) {
    use pretty_hex::*;

    let hex_dump = format!("{:?}", data.to_vec().hex_dump());

    log::trace!("{}", "-".repeat(76));

    for line in hex_dump.lines() {
        log::trace!("{}", line);
    }

    log::trace!("{}", "-".repeat(76));
}

pub(crate) fn trace() -> bool {
    log::max_level() >= log::Level::Trace
}
