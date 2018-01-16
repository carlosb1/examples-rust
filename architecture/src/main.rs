use chrono::prelude::*;

struct Ship {
    pub name String,
    pub location String
}

impl Ship {
    pub fn new (name: String, location: String) {
        return Ship {name, location};
    }
    pub fn handleDeparture (event: DepartureEvent) {
        println!("go");
    
    }
}

struct ShippingEvent {
    pub ocurred: DateTime<Utc>,
    pub recorded: DateTime<Utc>
}

impl ShippingEvent {
    pub fn new (ocurred: DateTime<Utc>, recorded: DateTime<Utc>) {
        return ShippingEvent {ocurred, recorded};
    }
}


struct DepartureEvent {
    pub ShippingEvent shippingEvent,
    pub ship Ship,
    pub port String
}

impl DepartureEvent {
    pub fn new (ocurred: DateTime<Utc>, recorded: DateTime<Utc>,
                ship: Ship, port: String) { 
        return DepartureEvent {ShippingEvent::new(ocurred, recorded), ship, port};
    }
    pub fn process () {
        self.ship.handleDeparture();
    }
}

struct EventProcessor {
}

impl TrackingService {
    pub fn process (departureEvent: DepartureEvent) {
        departureEvent.process();
    }
}


fn main () {

}


