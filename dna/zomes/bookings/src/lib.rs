use hdk3::prelude::*;

mod booking;
mod booking_request;
mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![
    Path::entry_def(),
    booking::Booking::entry_def(),
    booking_request::BookingRequest::entry_def()
];

/** Booking requests **/

#[hdk_extern]
pub fn request_to_book_resource(
    request_to_book_resource_input: booking_request::RequestToBookResourceInput,
) -> ExternResult<EntryHash> {
    booking_request::request_to_book_resource(request_to_book_resource_input)
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetBookingRequestsForResourceOutput(Vec<(EntryHash, booking_request::BookingRequest)>);
#[hdk_extern]
pub fn get_booking_requests_for_resource(
    resource_hash: EntryHash,
) -> ExternResult<GetBookingRequestsForResourceOutput> {
    let booking_requests = booking_request::get_booking_requests_for_resource(resource_hash)?;

    Ok(GetBookingRequestsForResourceOutput(booking_requests))
}

#[hdk_extern]
pub fn accept_booking_request(booking_request_hash: EntryHash) -> ExternResult<EntryHash> {
    booking_request::accept_booking_request(booking_request_hash)
}

/** Bookings  **/

#[hdk_extern]
pub fn create_booking_for_request(booking_request_hash: EntryHash) -> ExternResult<EntryHash> {
    booking::create_booking_for_request(booking_request_hash)
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetBookingsForResourceOutput(Vec<(EntryHash, booking::Booking)>);
#[hdk_extern]
pub fn get_bookings_for_resource(
    resource_hash: EntryHash,
) -> ExternResult<GetBookingsForResourceOutput> {
    let bookings = booking::get_bookings_for_resource(resource_hash)?;

    Ok(GetBookingsForResourceOutput(bookings))
}
