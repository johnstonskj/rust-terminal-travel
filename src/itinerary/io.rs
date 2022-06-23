/*!
One-line description.
More detailed description, with
# Example
 */

use crate::error::Result;
use crate::itinerary::Itinerary;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn from_str(s: &str) -> Result<Itinerary> {
    let i: Itinerary = serde_yaml::from_str(s)?;
    Ok(i)
}

pub fn from_reader(r: impl Read) -> Result<Itinerary> {
    let i: Itinerary = serde_yaml::from_reader(r)?;
    Ok(i)
}

pub fn from_file(p: &Path) -> Result<Itinerary> {
    let f = File::open(p)?;
    let i: Itinerary = from_reader(f)?;
    Ok(i)
}

// ------------------------------------------------------------------------------------------------

pub fn to_string(i: &Itinerary) -> Result<String> {
    let s = serde_yaml::to_string(i)?;
    Ok(s)
}

pub fn to_writer(i: &Itinerary, p: impl Write) -> Result<()> {
    serde_yaml::to_writer(p, i)?;
    Ok(())
}

pub fn to_file(i: &Itinerary, p: &Path) -> Result<()> {
    let f = File::open(p)?;
    serde_yaml::to_writer(f, i)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{from_str, to_string};
    use crate::itinerary::{
        Accomodation, AirCarrierCode, AirportCode, Date, Flight, Itinerary, Transport,
        TransportKind, TravelRecord,
    };
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    fn example_itinerary() -> Itinerary {
        Itinerary {
            identifier: "2022-06".to_string(),
            name: "Playa del Carmen".to_string(),
            travelers: vec!["Simon".to_string()],
            items: vec![
                Flight {
                    number: 594,
                    record: TravelRecord {
                        provider: AirCarrierCode::from_str("DL").unwrap(),
                        confirmation: Some("GROVQY".to_string()),
                        start_date_time: Date::from_ymd(2022, 6, 24).and_hms(7, 15, 00),
                        start_location: AirportCode::from_str("SEA").unwrap(),
                        end_date_time: Date::from_ymd(2022, 6, 24).and_hms(15, 3, 00),
                        end_location: Some(AirportCode::from_str("CUN").unwrap()),
                    },
                }
                .into(),
                Flight {
                    number: 604,
                    record: TravelRecord {
                        provider: AirCarrierCode::from_str("DL").unwrap(),
                        confirmation: Some("GROVQY".to_string()),
                        start_date_time: Date::from_ymd(2022, 7, 5).and_hms(14, 3, 00),
                        start_location: AirportCode::from_str("CUN").unwrap(),
                        end_date_time: Date::from_ymd(2022, 7, 5).and_hms(20, 23, 00),
                        end_location: Some(AirportCode::from_str("SEA").unwrap()),
                    },
                }
                .into(),
                Transport {
                    record: TravelRecord {
                        provider: "CARM".to_string(),
                        confirmation: None,
                        start_date_time: Date::from_ymd(2022, 6, 24).and_hms(16, 00, 00),
                        start_location: "Cancun airport".to_string(),
                        end_date_time: Date::from_ymd(2022, 6, 24).and_hms(17, 00, 00),
                        end_location: Some("The Elements, Playa del Carmen".to_string()),
                    },
                    kind: Some(TransportKind::Shuttle),
                }
                .into(),
                Transport {
                    record: TravelRecord {
                        provider: "CARM".to_string(),
                        confirmation: Some("GROVQY".to_string()),
                        start_date_time: Date::from_ymd(2022, 7, 5).and_hms(11, 00, 00),
                        start_location: "The Elements, Playa del Carmen".to_string(),
                        end_date_time: Date::from_ymd(2022, 7, 5).and_hms(12, 00, 00),
                        end_location: Some("Cancun airport".to_string()),
                    },
                    kind: Some(TransportKind::Shuttle),
                }
                .into(),
                Accomodation {
                    record: TravelRecord {
                        provider: "bric".to_string(),
                        confirmation: Some("6015334".to_string()),
                        start_date_time: Date::from_ymd(2022, 6, 26).and_hms(16, 00, 00),
                        start_location: "Garden house 11, The Elements, Playa dl Carmen."
                            .to_string(),
                        end_date_time: Date::from_ymd(2022, 7, 5).and_hms(11, 00, 00),
                        end_location: None,
                    },
                }
                .into(),
            ],
        }
    }

    const EXAMPLE_ITINERARY_STR: &str = r##"---
identifier: 2022-06
name: Playa del Carmen
travelers:
  - Simon
items:
  - flight:
      number: 594
      record:
        provider: DL
        confirmation: GROVQY
        start_date_time: "2022-06-24T07:15:00"
        start_location: SEA
        end_date_time: "2022-06-24T15:03:00"
        end_location: CUN
  - flight:
      number: 604
      record:
        provider: DL
        confirmation: GROVQY
        start_date_time: "2022-07-05T14:03:00"
        start_location: CUN
        end_date_time: "2022-07-05T20:23:00"
        end_location: SEA
  - transport:
      record:
        provider: CARM
        start_date_time: "2022-06-24T16:00:00"
        start_location: Cancun airport
        end_date_time: "2022-06-24T17:00:00"
        end_location: "The Elements, Playa del Carmen"
      kind: shuttle
  - transport:
      record:
        provider: CARM
        confirmation: GROVQY
        start_date_time: "2022-07-05T11:00:00"
        start_location: "The Elements, Playa del Carmen"
        end_date_time: "2022-07-05T12:00:00"
        end_location: Cancun airport
      kind: shuttle
  - accomodation:
      record:
        provider: bric
        confirmation: "6015334"
        start_date_time: "2022-06-26T16:00:00"
        start_location: "Garden house 11, The Elements, Playa dl Carmen."
        end_date_time: "2022-07-05T11:00:00"
"##;

    #[test]
    fn test_serialization() {
        let serial = to_string(&example_itinerary()).unwrap();
        println!("{}", serial);
        assert_eq!(serial, EXAMPLE_ITINERARY_STR.to_string());
    }

    #[test]
    fn test_deserialization() {
        let itinerary = from_str(EXAMPLE_ITINERARY_STR).unwrap();
        println!("{:#?}", itinerary);
        assert_eq!(itinerary, example_itinerary());
    }
}
