use crate::{booking, utils};
use hdk3::prelude::timestamp::Timestamp;
use hdk3::prelude::*;

#[hdk_entry(id = "booking_request", visibility = "public")]
#[derive(Clone)]
pub struct BookingRequest {
    pub requestor_pub_key: AgentPubKey,
    pub resource_hash: EntryHash,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub maybe_calendar_event_hash: Option<EntryHash>,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct RequestToBookResourceInput {
    resource_hash: EntryHash,
    start_time: Timestamp,
    end_time: Timestamp,
    maybe_calendar_event_hash: Option<EntryHash>,
}
pub fn request_to_book_resource(
    request_to_book_resource_input: RequestToBookResourceInput,
) -> ExternResult<EntryHash> {
    let agent_info = agent_info!()?;

    let booking_request = BookingRequest {
        requestor_pub_key: agent_info.agent_latest_pubkey,
        resource_hash: request_to_book_resource_input.resource_hash.clone(),
        start_time: request_to_book_resource_input.start_time.clone(),
        end_time: request_to_book_resource_input.end_time.clone(),
        maybe_calendar_event_hash: request_to_book_resource_input
            .maybe_calendar_event_hash
            .clone(),
    };

    create_entry!(booking_request.clone())?;

    let booking_request_hash = hash_entry!(booking_request)?;

    create_link!(
        request_to_book_resource_input.resource_hash,
        booking_request_hash.clone(),
        utils::link_tag("resource->booking_request")?
    )?;

    Ok(booking_request_hash)
}

pub fn get_booking_requests_for_resource(
    resource_hash: EntryHash,
) -> ExternResult<Vec<(EntryHash, BookingRequest)>> {
    let links = get_links!(resource_hash, utils::link_tag("resource->booking_request")?)?;

    links
        .into_inner()
        .iter()
        .map(|link| utils::try_get_and_convert::<BookingRequest>(link.target.clone()))
        .collect()
}

pub fn accept_booking_request(booking_request_hash: EntryHash) -> ExternResult<EntryHash> {
    let booking_hash = booking::create_booking_for_request(booking_request_hash.clone())?;

    // remove_request_links(booking_request_hash)?;

    Ok(booking_hash)
}

/* pub fn remove_request_links(booking_request_hash: EntryHash) -> ExternResult<()> {
    let booking_request: BookingRequest = utils::try_get_and_convert(booking_request_hash.clone())?;

    let link_details = get_link_details!(
        booking_request.resource_hash,
        utils::link_tag("resource->booking_request")?
    )?;

    link_details
        .into_inner()
        .iter()
        .map(|link_detail| {
            let header_hash = Element::from(link_detail.0).as_hash();

            delete_link!(header_hash)?;

            Ok(())
        })
        .collect::<ExternResult<Vec<()>>>();
    Ok(())
}
 */
