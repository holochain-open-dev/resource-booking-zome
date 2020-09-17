use crate::utils;
use hdk3::prelude::*;

#[hdk_entry(id = "meeting_room", visibility = "public")]
#[derive(Clone)]
pub struct MeetingRoom {
    name: String,
    description: String,
    owner: AgentPubKey,
}

pub fn create_meeting_room(name: String, description: String) -> ExternResult<EntryHash> {
    let agent_info = agent_info!()?;

    let meeting_room = MeetingRoom {
        name,
        description,
        owner: agent_info.agent_latest_pubkey,
    };

    let meeting_room_hash = hash_entry!(meeting_room.clone())?;

    create_entry!(meeting_room.clone())?;

    let anchor_address = all_meetings_anchor_address()?;

    create_link!(anchor_address, meeting_room_hash.clone())?;

    Ok(meeting_room_hash)
}

pub fn get_all_meeting_rooms() -> ExternResult<Vec<MeetingRoom>> {
    let anchor_address = all_meetings_anchor_address()?;

    let links = get_links!(anchor_address)?;

    links
        .into_inner()
        .iter()
        .map(|link| utils::try_get_and_convert::<MeetingRoom>(link.target.clone()))
        .collect::<ExternResult<Vec<MeetingRoom>>>()
}

pub fn get_resource_authorities_at_time(
    resource_hash: EntryHash,
    _time: timestamp::Timestamp,
) -> ExternResult<Vec<AgentPubKey>> {
    let meeting_room = utils::try_get_and_convert::<MeetingRoom>(resource_hash)?;
    Ok(vec![meeting_room.owner])
}

/** Private helpers **/

fn all_meetings_anchor_address() -> ExternResult<EntryHash> {
    anchor("meeting_rooms".into(), "".into())
}