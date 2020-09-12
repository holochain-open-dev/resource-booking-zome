use hdk3::prelude::*;

pub trait BookableResource {
    fn get_resource_authorities_at_time(
        resource_address: EntryHash,
        time: timestamp::Timestamp,
    ) -> ExternResult<Vec<AgentPubKey>>;
}
