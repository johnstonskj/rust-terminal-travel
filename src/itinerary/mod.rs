/*!
One-line description.

More detailed description, with

# Example

*/

use_required!();
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Date = chrono::NaiveDate;

pub type Time = chrono::NaiveTime;

pub type DateTime = chrono::NaiveDateTime;

pub type Duration = chrono::Duration;

is_valid_newstring!(
    AirCarrierCode,
    is_valid_air_carrier_code,
    Deserialize,
    Serialize
);

is_valid_newstring!(AirportCode, is_valid_airport_code, Deserialize, Serialize);

pub type FlightNumber = u16;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Itinerary {
    identifier: String,
    name: String,
    travelers: Vec<String>,
    items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Booking {
    agency: String,
    confirmation: String,
    items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Flight(Flight),
    Transport(Transport),
    Vehicle(Vehicle),
    Accomodation(Accomodation),
    Event(Event),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    number: FlightNumber,
    record: TravelRecord<AirCarrierCode, AirportCode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Ferry,
    Shuttle,
    Taxi,
    Train,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transport {
    record: TravelRecord<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<TransportKind>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    record: TravelRecord<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Accomodation {
    record: TravelRecord<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    record: TravelRecord<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TravelRecord<P, L> {
    provider: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation: Option<String>,
    start_date_time: DateTime,
    start_location: L,
    end_date_time: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_location: Option<L>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

impl Itinerary {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn travelers(&self) -> impl Iterator<Item = &String> {
        self.travelers.iter()
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl Item {
    pub fn is_flight(&self) -> bool {
        matches!(&self, Item::Flight(_))
    }

    pub fn as_flight(&self) -> Option<&Flight> {
        match self {
            Item::Flight(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_transport(&self) -> bool {
        matches!(&self, Item::Transport(_))
    }

    pub fn as_transport(&self) -> Option<&Transport> {
        match self {
            Item::Transport(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_accomodation(&self) -> bool {
        matches!(&self, Item::Accomodation(_))
    }

    pub fn as_accomodation(&self) -> Option<&Accomodation> {
        match self {
            Item::Accomodation(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_vehicle(&self) -> bool {
        matches!(&self, Item::Vehicle(_))
    }

    pub fn as_vehicle(&self) -> Option<&Vehicle> {
        match self {
            Item::Vehicle(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_event(&self) -> bool {
        matches!(&self, Item::Event(_))
    }

    pub fn as_event(&self) -> Option<&Event> {
        match self {
            Item::Event(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Flight> for Item {
    fn from(v: Flight) -> Self {
        Self::Flight(v)
    }
}

impl Flight {
    pub fn carrier(&self) -> &AirCarrierCode {
        &self.record.provider
    }

    pub fn number(&self) -> &FlightNumber {
        &self.number
    }

    pub fn flight_number_string(&self) -> String {
        format!("{}{}", self.carrier(), self.number())
    }

    pub fn record_locator(&self) -> &String {
        self.record.confirmation.as_ref().unwrap()
    }

    pub fn departure_date_time(&self) -> &DateTime {
        &self.record.start_date_time
    }

    pub fn departure_airport(&self) -> &AirportCode {
        &self.record.start_location
    }

    pub fn arrival_date_time(&self) -> &DateTime {
        &self.record.end_date_time
    }

    pub fn arrival_airport(&self) -> &AirportCode {
        self.record.end_location.as_ref().unwrap()
    }

    pub(crate) fn inner(&self) -> &TravelRecord<AirCarrierCode, AirportCode> {
        &self.record
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for TransportKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransportKind::Ferry => "ferry",
                TransportKind::Shuttle => "shuttle",
                TransportKind::Taxi => "taxi",
                TransportKind::Train => "train",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Transport> for Item {
    fn from(v: Transport) -> Self {
        Self::Transport(v)
    }
}

impl Transport {
    pub fn company_or_agency(&self) -> &String {
        &self.record.provider
    }

    pub fn confirmation(&self) -> Option<&String> {
        self.record.confirmation.as_ref()
    }

    pub fn departure_date_time(&self) -> &DateTime {
        &self.record.start_date_time
    }

    pub fn departure_address(&self) -> &String {
        &self.record.start_location
    }

    pub fn arrival_date_time(&self) -> &DateTime {
        &self.record.end_date_time
    }

    pub fn arrival_address(&self) -> &String {
        self.record.end_location.as_ref().unwrap()
    }

    pub fn kind(&self) -> Option<&TransportKind> {
        self.kind.as_ref()
    }

    pub(crate) fn inner(&self) -> &TravelRecord<String, String> {
        &self.record
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Vehicle> for Item {
    fn from(v: Vehicle) -> Self {
        Self::Vehicle(v)
    }
}

impl Vehicle {
    pub fn rental_agency(&self) -> &String {
        &self.record.provider
    }

    pub fn confirmation(&self) -> Option<&String> {
        self.record.confirmation.as_ref()
    }

    pub fn pickup_date_time(&self) -> &DateTime {
        &self.record.start_date_time
    }

    pub fn pickup_address(&self) -> &String {
        &self.record.start_location
    }

    pub fn dropoff_date_time(&self) -> &DateTime {
        &self.record.end_date_time
    }

    pub fn dropoff_address(&self) -> &String {
        self.record.end_location.as_ref().unwrap()
    }

    pub(crate) fn inner(&self) -> &TravelRecord<String, String> {
        &self.record
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Accomodation> for Item {
    fn from(v: Accomodation) -> Self {
        Self::Accomodation(v)
    }
}

impl Accomodation {
    pub fn company(&self) -> &String {
        &self.record.provider
    }

    pub fn confirmation(&self) -> Option<&String> {
        self.record.confirmation.as_ref()
    }

    pub fn address(&self) -> &String {
        &self.record.start_location
    }

    pub fn checkin_date_time(&self) -> &DateTime {
        &self.record.start_date_time
    }

    pub fn checkout_date_time(&self) -> &DateTime {
        &self.record.end_date_time
    }

    pub(crate) fn inner(&self) -> &TravelRecord<String, String> {
        &self.record
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Event> for Item {
    fn from(v: Event) -> Self {
        Self::Event(v)
    }
}

impl Event {
    pub fn company(&self) -> &String {
        &self.record.provider
    }

    pub fn confirmation(&self) -> Option<&String> {
        self.record.confirmation.as_ref()
    }

    pub fn address(&self) -> &String {
        &self.record.start_location
    }

    pub fn start_date_time(&self) -> &DateTime {
        &self.record.start_date_time
    }

    pub fn end_date_time(&self) -> &DateTime {
        &self.record.end_date_time
    }

    pub(crate) fn inner(&self) -> &TravelRecord<String, String> {
        &self.record
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn is_valid_air_carrier_code(s: &str) -> bool {
    (s.len() == 2 || s.len() == 3) && s.chars().all(|c| c.is_ascii_alphanumeric())
}

#[inline]
fn is_valid_airport_code(s: &str) -> bool {
    s.len() == 3 && s.chars().all(|c| c.is_ascii_alphabetic())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod io;

pub mod visitor;

pub mod display;

pub mod export;

pub mod edit;
