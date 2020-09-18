use hdk3::prelude::*;

mod meeting_room;
mod utils;
use meeting_room::MeetingRoom;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![MeetingRoom::entry_def(), Path::entry_def()];

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct CreateMeetingRoomInput {
    name: String,
    description: String,
}
#[hdk_extern]
pub fn create_meeting_room(input: CreateMeetingRoomInput) -> ExternResult<EntryHash> {
    meeting_room::create_meeting_room(input.name, input.description)
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetAllMeetingRoomsOutput(Vec<MeetingRoom>);
#[hdk_extern]
pub fn get_all_meeting_rooms(_: ()) -> ExternResult<GetAllMeetingRoomsOutput> {
    let meeting_rooms = meeting_room::get_all_meeting_rooms()?;
    Ok(GetAllMeetingRoomsOutput(meeting_rooms))
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetResourceAuthoritiesInput {
    resource_hash: EntryHash,
    timestamp: timestamp::Timestamp,
}
#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetResourceAuthoritiesOutput(Vec<AgentPubKey>);
#[hdk_extern]
pub fn get_resource_authorities_at_time(
    input: GetResourceAuthoritiesInput,
) -> ExternResult<GetResourceAuthoritiesOutput> {
    let authorities =
        meeting_room::get_resource_authorities_at_time(input.resource_hash, input.timestamp)?;

    Ok(GetResourceAuthoritiesOutput(authorities))
}
