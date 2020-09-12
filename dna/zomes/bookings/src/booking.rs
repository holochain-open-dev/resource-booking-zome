use crate::{utils, booking_request::BookingRequest};
use hdk3::prelude::timestamp::Timestamp;
use hdk3::prelude::*;

#[hdk_entry(id = "booking", visibility = "public")]
#[derive(Clone)]
pub struct Booking {
    pub booking_request_hash: EntryHash,
    pub created_at: Timestamp
}

pub fn create_booking_for_request(booking_request_hash: EntryHash) -> ExternResult<EntryHash> {
    let booking_request: BookingRequest = utils::try_get_and_convert(booking_request_hash.clone())?;

    let time = sys_time!()?;
    let now = Timestamp(time.as_secs() as i64, time.subsec_nanos());

    let booking = Booking {
        booking_request_hash,
        created_at: now
    };

    commit_entry!(booking.clone())?;

    let booking_hash = entry_hash!(booking.clone())?;

    link_entries!(
        booking_request.resource_hash,
        booking_hash.clone(),
        utils::link_tag("resource->booking")?
    )?;

    Ok(booking_hash)
}

pub fn get_bookings_for_resource(resource_hash: EntryHash) -> ExternResult<Vec<Booking>> {
    let links = get_links!(resource_hash, utils::link_tag("resource->booking")?)?;

    links
        .into_inner()
        .iter()
        .map(|link| utils::try_get_and_convert(link.target.clone()))
        .collect()
}
