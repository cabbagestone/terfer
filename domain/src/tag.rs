use uuid::Uuid;
use crate::instance::{Instance, Instanced, InstanceError, InstanceList};
use crate::version::VersionLevel;

pub struct Tag {
    id: String,
    instances: InstanceList<TagInstance>,
}

impl Tag {
    pub fn new(value: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            instances: InstanceList::new(Vec::from([TagInstance::new(value)])),
        }
    }
    
    pub fn edit(&mut self, value: String, note: String) -> Result<(), TagError> {
        let tag_instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(TagError::EditEmptyTag),
        };
        
        let new_instance = tag_instance.get_instance().create_child_instance(note, VersionLevel::Major);
        self.instances.add(TagInstance::with_instance(value, new_instance))?;
        
        Ok(())
    }
    
    pub fn delete(&mut self, note: Option<String>) -> Result<(), TagError> {
        let tag_instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(TagError::EditEmptyTag),
        };
        
        let new_instance = tag_instance.get_instance().create_deletion_instance(note);
        self.instances.add(TagInstance::with_instance(tag_instance.value.clone(), new_instance))?;
        
        Ok(())
    }
    
    pub fn restore(&mut self, note: Option<String>) -> Result<(), TagError> {
        let tag_instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(TagError::EditEmptyTag),
        };
        
        let new_instance = tag_instance.get_instance().create_restoration_instance(note);
        self.instances.add(TagInstance::with_instance(tag_instance.value.clone(), new_instance))?;
        
        Ok(())
    }
    
    pub fn get_id(&self) -> &str {
        &self.id
    }
    
    pub fn get_value(&self) -> Result<String, TagError> {
        match self.instances.latest() {
            Some(instance) => Ok(instance.value.clone()),
            None => Err(TagError::RetrieveEmptyTag),
        }
    }
}

#[derive(Debug)]
pub enum TagError {
    EditEmptyTag,
    RetrieveEmptyTag,
    Instance(InstanceError),
}

impl std::error::Error for TagError {}

impl From<InstanceError> for TagError {
    fn from(e: InstanceError) -> Self {
        TagError::Instance(e)
    }
}

impl std::fmt::Display for TagError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TagError::EditEmptyTag => write!(f, "Cannot edit an empty tag"),
            TagError::Instance(e) => write!(f, "Tag Instance Error: {}", e),
            TagError::RetrieveEmptyTag => write!(f, "Cannot retrieve an empty tag"),
        }
    }
}

struct TagInstance {
    id: String,
    value: String,
    instance: Instance
}

impl TagInstance {
    pub fn new(value: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            value,
            instance: Instance::create_initial_instance(VersionLevel::Major),
        }
    }
    
    pub fn with_instance(value: String, instance: Instance) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            value,
            instance,
        }
    }
}

impl Instanced for TagInstance {
    fn get_instance(&self) -> &Instance {
        &self.instance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::Version;
    
    struct TestTag {
        tag: Tag,
    }

    impl Instanced for TestTag {
        fn get_instance(&self) -> &Instance {
            self.tag.instances.latest().unwrap().get_instance()
        }
    }
    
    #[test]
    fn test_tag() {
        let mut tag = TestTag {
            tag: Tag::new(String::from("Test Tag")),
        };
        
        assert_eq!(tag.get_instance().get_version(), &Version::new(1, 0, 0));
        
        tag.tag.edit(String::from("Test Tag 2"), String::from("Test Change")).unwrap();
        assert_eq!(tag.get_instance().get_version(), &Version::new(2, 0, 0));
        
        tag.tag.delete(Some(String::from("Delete Tag"))).unwrap();
        assert_eq!(tag.get_instance().get_version(), &Version::new(3, 0, 0));
        
        tag.tag.restore(Some(String::from("Restore Tag"))).unwrap();
        assert_eq!(tag.get_instance().get_version(), &Version::new(4, 0, 0));
        
        assert_eq!(tag.tag.get_value().unwrap(), "Test Tag 2");
    }
}