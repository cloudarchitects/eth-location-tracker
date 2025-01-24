use log::{info, LevelFilter};
use beacons::{BEACONS};

fn main() {

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(LevelFilter::Info);

    for b in BEACONS.iter() {
        info!("{:?}", b);
    }

    info!("Hello, world!");
}
