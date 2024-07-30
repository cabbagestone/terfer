use jiff::Zoned;
use crate::version::Version;

pub struct Instance {
    datetime: Zoned,
    change_note: String,
    is_deletion: bool,
    version: Version,
}

pub struct InstanceList<T> {
    instances: Vec<T>,
}