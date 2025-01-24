use std::string::ToString;
use positioning::geographic::Position;

#[derive(Debug, Clone)]
pub struct Id {
    pub uuid:  &'static str,
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, Clone)]
pub struct Beacon {
    pub id: Id,
    pub position: Position,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub building: &'static str, // Changed to &str
    pub floor: &'static str,
    pub room: &'static str,
}

static ETH_UUID: &str = "58793564-459c-548d-bfcc-367ffd4fcd70";
pub static BEACONS: &[Beacon] = &[
    #[cfg(feature = "WPW")]
    Beacon {
        id: Id { uuid: ETH_UUID, major: 0, minor: 2686 },
        position: Position { lat: 47.37874008859311, lon: 8.542589723716747 },
        location: Location { building: "WPW", floor: "B", room: "6" },
    },
];
