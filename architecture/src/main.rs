use chrono::prelude::*;

struct Ship {
    pub name String,
    pub location String
}

struct ShippingEvent {
    pub ocurred: DateTime<Utc>,
    pub recorded: DateTime<Utc>
}

struct ArrivalEvent: ShippingEvent {
    pub ship Ship,
    pub port String

}

struct DepartureEvent: ShippingEvent {
    pub ship Ship,
    pub port String
}

struct TrackingService {
}

impl TrackingService {
    pub fn recordArrival (ship: Ship, port: Port) {
    }
    pub fn recordDeparture (ship :Ship, port: Port) {
    }
}


fn main () {

}


