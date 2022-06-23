/*!
One-line description.
More detailed description, with
# Example
 */

use crate::error::Result;
use crate::itinerary::visitor::{visit_ordered_itinerary, ItineraryVisitor, VisitOrder};
use crate::itinerary::Itinerary;
use crate::itinerary::{Accomodation, Event, Flight, Transport, TravelRecord, Vehicle};
use console::Term;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum DisplayFormat {
    NuTable,
    Cards,
    Indented,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn display_itinerary(itinerary: &Itinerary, format: DisplayFormat) -> Result<()> {
    match format {
        DisplayFormat::NuTable => {
            let visitor = NuTable(Term::stdout(), Default::default());
            visit_ordered_itinerary(itinerary, &visitor, VisitOrder::AsIs);
        }
        DisplayFormat::Cards => {
            let visitor = Cards(Term::stdout());
        }
        DisplayFormat::Indented => {
            let visitor = Indented(Term::stdout());
        }
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct NuTable(Term, RefCell<Vec<BTreeMap<String, String>>>);

#[derive(Debug)]
pub struct Cards(Term);

#[derive(Debug)]
pub struct Indented(Term);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DisplayFormat {
    fn default() -> Self {
        Self::NuTable
    }
}

impl Display for DisplayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DisplayFormat::NuTable => "nushell",
                DisplayFormat::Cards => "cards",
                DisplayFormat::Indented => "indented",
            }
        )
    }
}

impl FromStr for DisplayFormat {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "nu" | "nutable" => Ok(Self::NuTable),
            "cards" => Ok(Self::Cards),
            "indented" => Ok(Self::Indented),
            _ => Err(()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl ItineraryVisitor for NuTable {
    fn flight_departs(&self, flight: &Flight) -> Result<()> {
        let record = flight.inner();
        let row = to_hashmap("flight", record, &flight.number().to_string(), "");
        let _ = self.1.borrow_mut().push(row);
        Ok(())
    }

    fn transport_departs(&self, transport: &Transport) -> Result<()> {
        let record = transport.inner();
        let row = to_hashmap(
            "transport",
            record,
            "",
            &transport.kind().map(|v| v.to_string()).unwrap_or_default(),
        );
        let _ = self.1.borrow_mut().push(row);
        Ok(())
    }

    fn vehicle_pickup(&self, vehicle: &Vehicle) -> Result<()> {
        let record = vehicle.inner();
        let row = to_hashmap("vehicle", record, "", "");
        let _ = self.1.borrow_mut().push(row);
        Ok(())
    }

    fn accomodation_checkin(&self, accomodation: &Accomodation) -> Result<()> {
        let record = accomodation.inner();
        let row = to_hashmap("accomodation", record, "", "");
        let _ = self.1.borrow_mut().push(row);
        Ok(())
    }

    fn event_start(&self, event: &Event) -> Result<()> {
        let record = event.inner();
        let row = to_hashmap("event", record, "", "");
        let _ = self.1.borrow_mut().push(row);
        Ok(())
    }

    fn end(&self) -> Result<()> {
        let inner = self.1.replace(Default::default());
        self.0.write_line(&serde_json::to_string(&inner)?)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl ItineraryVisitor for Cards {}

// ------------------------------------------------------------------------------------------------

impl ItineraryVisitor for Indented {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn to_hashmap<P, L>(
    item_type: &str,
    record: &TravelRecord<P, L>,
    flight_number: &str,
    transport_kind: &str,
) -> BTreeMap<String, String>
where
    P: Display,
    L: Display,
{
    let row: BTreeMap<String, String> = [
        ("__type".to_string(), item_type.to_string()),
        ("provider".to_string(), record.provider.to_string()),
        (
            "confirmation".to_string(),
            record
                .confirmation
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        ),
        (
            "start_date_time".to_string(),
            record.start_date_time.to_string(),
        ),
        (
            "start_location".to_string(),
            record.start_location.to_string(),
        ),
        (
            "end_date_time".to_string(),
            record.end_date_time.to_string(),
        ),
        (
            "end_location".to_string(),
            record
                .end_location
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        ),
        ("flight_number".to_string(), flight_number.to_string()),
        ("transport_kind".to_string(), transport_kind.to_string()),
    ]
    .into();
    row
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
