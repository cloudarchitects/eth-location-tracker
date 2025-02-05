use crate::bluetooth::ibeacon;
use crossbeam_channel::Sender;
use esp32_nimble::{BLEAdvertisedData, BLEAdvertisedDevice, BLEDevice, BLEScan};
use log::{debug, error};
use positioning::beacon::BeaconId;
use positioning::signal::Signal;

pub struct Scanner {
    scan_time_ms: i32,
    scan_interval_ms: u16,
    scan_window_ms: u16,
}

const ETH_BEACON_UUID: &str = "58793564-459c-548d-bfcc-367ffd4fcd70";

impl Scanner {
    pub fn new(scan_time_ms: i32, scan_interval_ms: u16, scan_window_ms: u16) -> Self {
        Scanner {
            scan_time_ms,
            scan_interval_ms,
            scan_window_ms,
        }
    }

    pub async fn scan_indefinit(&self, tx: Sender<Signal<BeaconId>>) {
        let ble_device = BLEDevice::take();
        let mut ble_scan = BLEScan::new();
        loop {
            let _ = ble_scan
                .active_scan(true)
                .interval(self.scan_interval_ms)
                .window(self.scan_window_ms)
                .start(
                    ble_device,
                    self.scan_time_ms,
                    |device: &BLEAdvertisedDevice, data: BLEAdvertisedData<&[u8]>| {
                        if let Some(ibeacon) = ibeacon::read_bytes(data.payload())
                            .take_if(|ibeacon| ETH_BEACON_UUID.eq(&ibeacon.uuid))
                        {
                            let uuid = ibeacon.uuid.as_str();
                            let major = ibeacon.major;
                            let minor = ibeacon.minor;

                            if let Err(e) = tx.send(Signal::new(
                                BeaconId::new(uuid, major, minor),
                                ibeacon.power,
                                device.rssi(),
                            )) {
                                error!("Failed to send signal: {}", e);
                            }
                        }

                        let result: Option<String> = None;
                        result
                    },
                )
                .await;
        }
    }
}

impl Drop for Scanner {
    fn drop(&mut self) {
        debug!("dropping Scanner")
    }
}
