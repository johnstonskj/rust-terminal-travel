/*!
One-line description.
More detailed description, with
# Example
 */

use super::{Accomodation, Event, Flight, Itinerary, Transport, Vehicle};
use crate::error::Result;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait ItineraryVisitor {
    fn start(&self) -> Result<()> {
        Ok(())
    }

    fn identifier(&self, id: &str) -> Result<()> {
        Ok(())
    }

    fn name(&self, name: &str) -> Result<()> {
        Ok(())
    }

    fn traveler(&self, name: &str) -> Result<()> {
        Ok(())
    }

    fn flight_departs(&self, flight: &Flight) -> Result<()> {
        Ok(())
    }

    fn flight_arrives(&self, flight: &Flight) -> Result<()> {
        Ok(())
    }

    fn transport_departs(&self, transport: &Transport) -> Result<()> {
        Ok(())
    }

    fn transport_arrives(&self, transport: &Transport) -> Result<()> {
        Ok(())
    }

    fn vehicle_pickup(&self, vehicle: &Vehicle) -> Result<()> {
        Ok(())
    }

    fn vehicle_dropoff(&self, vehicle: &Vehicle) -> Result<()> {
        Ok(())
    }

    fn accomodation_checkin(&self, accomodation: &Accomodation) -> Result<()> {
        Ok(())
    }

    fn accomodation_checkout(&self, accomodation: &Accomodation) -> Result<()> {
        Ok(())
    }

    fn event_start(&self, event: &Event) -> Result<()> {
        Ok(())
    }

    fn event_end(&self, event: &Event) -> Result<()> {
        Ok(())
    }

    fn end(&self) -> Result<()> {
        Ok(())
    }
}

pub enum VisitOrder {
    StartEnd,
    Booking,
    AsIs,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn visit_itinerary(itinerary: &Itinerary, visitor: &impl ItineraryVisitor) -> Result<()> {
    visit_ordered_itinerary(itinerary, visitor, VisitOrder::default())
}

pub fn visit_ordered_itinerary(
    itinerary: &Itinerary,
    visitor: &impl ItineraryVisitor,
    order: VisitOrder,
) -> Result<()> {
    visitor.start()?;

    visitor.identifier(itinerary.identifier())?;
    visitor.name(itinerary.name())?;

    match order {
        VisitOrder::StartEnd => todo!(),
        VisitOrder::Booking => todo!(),
        VisitOrder::AsIs => {
            for item in itinerary.items() {
                match item {
                    super::Item::Flight(v) => {
                        visitor.flight_departs(v)?;
                        visitor.flight_arrives(v)?;
                    }
                    super::Item::Transport(v) => {
                        visitor.transport_departs(v)?;
                        visitor.transport_arrives(v)?;
                    }
                    super::Item::Vehicle(v) => {
                        visitor.vehicle_pickup(v)?;
                        visitor.vehicle_dropoff(v)?;
                    }
                    super::Item::Accomodation(v) => {
                        visitor.accomodation_checkin(v)?;
                        visitor.accomodation_checkout(v)?;
                    }
                    super::Item::Event(v) => {
                        visitor.event_start(v)?;
                        visitor.event_end(v)?;
                    }
                }
            }
        }
    }

    visitor.end()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for VisitOrder {
    fn default() -> Self {
        Self::StartEnd
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
