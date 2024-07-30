use crate::instance::{Instance, InstanceList};

pub struct Tag {
    id: String,
    instances: InstanceList<TagInstance>,
}

pub struct TagInstance {
    id: String,
    value: String,
    instance_meta: Instance
}