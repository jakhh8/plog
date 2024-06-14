use plog::{debug, error, info, trace, warn};

fn main() {
    plog::init();

    trace!("trace!");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");
}
