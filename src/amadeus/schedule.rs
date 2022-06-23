/*!
One-line description.

More detailed description, with

# Example

*/

use crate::amadeus::execute::ResponseMetadata;
use crate::amadeus::GetRequest;
use crate::config::AppConfig;
use crate::itinerary::{AirCarrierCode, AirportCode, Date};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    carrier_code: AirCarrierCode,
    flight_number: u16,
    scheduled_departure_date: Date,
    operational_suffix: Option<char>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    meta: ResponseMetadata,
    data: Vec<DatedFlight>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatedFlight {
    _type: String,
    scheduled_departure_date: String,
    flight_designator: FlightDesignator,
    flight_points: Vec<FlightPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightDesignator {
    carrier_code: AirCarrierCode,
    flight_number: u16,
    operational_suffix: Option<char>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightPoint {
    iata_code: AirportCode,
    arrival: Option<ArrivalOrDeparture>,
    departure: Option<ArrivalOrDeparture>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrivalOrDeparture {
    timings: Vec<Timing>,
    terminal: Option<Terminal>,
    gate: Option<Gate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    qualifier: String,
    value: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    delays: Vec<Delay>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AircraftEquipment {
    aircraft_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delay {
    duration: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
    code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gate {
    main_gate: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    board_point_iata_code: AirportCode,
    off_point_iata_code: AirportCode,
    scheduled_segment_duration: String,
    partnership: Option<Partnership>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leg {
    board_point_iata_code: AirportCode,
    off_point_iata_code: AirportCode,
    scheduled_segment_duration: String,
    aircraft_equipment: Option<AircraftEquipment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partnership {
    operating_flight: FlightDesignator,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn flight_schedule_request(
    carrier_code: AirCarrierCode,
    flight_number: u16,
    scheduled_departure_date: Date,
    operational_suffix: Option<char>,
) -> Result<Request, crate::error::Error> {
    Ok(Request {
        carrier_code,
        flight_number,
        scheduled_departure_date,
        operational_suffix,
    })
}

make_api_call!(fetch_flight_schedule, Request, Response);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GetRequest for Request {
    const VERSION: u16 = 2;
    const PATH: &'static str = "/schedule/flights";

    fn has_query(&self) -> bool {
        true
    }

    fn query(&self) -> Option<String> {
        Some(format!(
            "carrierCode={}&flightNumber={}&scheduledDepartureDate={}{}",
            self.carrier_code,
            self.flight_number,
            self.scheduled_departure_date,
            match self.operational_suffix {
                None => String::new(),
                Some(operational_suffix) => format!("&operational_suffix={}", operational_suffix),
            }
        ))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
