use hdk3::prelude::*;

pub fn try_get_and_convert<T: TryFrom<SerializedBytes>>(
    entry_hash: EntryHash,
) -> ExternResult<(EntryHash, T)> {
    match get!(entry_hash.clone())? {
        Some(element) => Ok((entry_hash, try_from_element(element)?)),
        None => crate::error("Entry not found"),
    }
}

pub fn try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => try_from_entry::<T>(entry.clone()),
        _ => crate::error("Could not convert element"),
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => crate::error("Could not convert entry"),
        },
        _ => crate::error("Could not convert entry"),
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
