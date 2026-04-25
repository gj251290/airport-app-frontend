#[derive(Clone, Debug, PartialEq)]
pub enum TripType {
    RoundTrip,
    OneWay,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PassengerCount {
    pub adults: u8,
    pub children: u8,
    pub infants: u8,
}

impl Default for PassengerCount {
    fn default() -> Self {
        Self {
            adults: 1,
            children: 0,
            infants: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchParams {
    pub trip_type: TripType,
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub depart_date: Option<String>,
    pub return_date: Option<String>,
    pub passengers: PassengerCount,
}
