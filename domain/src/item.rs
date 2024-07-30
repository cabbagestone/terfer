use crate::instance::{Instance, InstanceList};
use crate::tag::Tag;

struct Item {
    id: String,
    instances: InstanceList<ItemInstance>,
    file_type: FileType,
    tags: Vec<Tag>,
}

struct ItemInstance {
    id: String,
    file_url: String,
    instance_meta: Instance,
}

enum FileType {
    Image,
    Video,
    Audio,
    Binary,
    Document,
    CodeFile,
    MarkdownNote,
    Archive,
    Specialized,
    Other
}