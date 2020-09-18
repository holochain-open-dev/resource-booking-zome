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

/** Validation **/

#[hdk_extern]
fn validate(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    if let Entry::App(entry_bytes) = entry.clone() {
        if entry_bytes.into_sb().bytes().len() > 1000 {
            return Ok(ValidateCallbackResult::Invalid("Too big".to_string()));
        }
    }

    if let Ok(meeting_room) = MeetingRoom::try_from(entry.clone()) {
        if meeting_room.name.len() > 20 {
            return Ok(ValidateCallbackResult::Invalid(
                "Meeting room name is too big".to_string(),
            ));
        }
    }

    return Ok(ValidateCallbackResult::Valid);
}
