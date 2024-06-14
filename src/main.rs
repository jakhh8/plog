use plog::{debug, error, info, trace, warn};

fn main() {
    plog::init().expect("Failed to initialize plog!");

    trace!("trace!");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");
}
