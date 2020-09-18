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
pub struct GetBookingRequestForResourceOutput(Vec<booking_request::BookingRequest>);
#[hdk_extern]
pub fn get_booking_requests_for_resource(
    resource_hash: EntryHash,
) -> ExternResult<GetBookingRequestForResourceOutput> {
    let booking_requests = booking_request::get_booking_requests_for_resource(resource_hash)?;

    Ok(GetBookingRequestForResourceOutput(booking_requests))
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
pub struct GetBookingForResourceOutput(Vec<booking::Booking>);
#[hdk_extern]
pub fn get_bookings_for_resource(
    resource_hash: EntryHash,
) -> ExternResult<GetBookingForResourceOutput> {
    let bookings = booking::get_bookings_for_resource(resource_hash)?;

    Ok(GetBookingForResourceOutput(bookings))
}

#[hdk_extern]
fn validate_link(
    validate_link_add_data: ValidateCreateLinkData,
) -> ExternResult<ValidateCreateLinkCallbackResult> {
    let _base_entry = validate_link_add_data.base;
    let _target_entry = validate_link_add_data.target;

    Ok(ValidateCreateLinkCallbackResult::Valid)
}
