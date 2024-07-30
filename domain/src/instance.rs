use std::cmp::PartialEq;
use std::fmt::Display;
use std::str::Matches;
use jiff::Zoned;
use crate::version::{Version, VersionLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    datetime: Zoned,
    change_note: String,
    instance_type: InstanceType,
    version: Version,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstanceType {
    Creation,
    Update,
    Deletion,
    Restoration,
}

impl Instance {
    pub fn create_initial_instance(version_level: VersionLevel) -> Self {
        Self {
            datetime: Zoned::now(),
            change_note: String::from("Instance Created"),
            instance_type: InstanceType::Creation,
            version: Version::new(0, 0, 0).create_child_version(version_level),
        }
    }
    
    pub fn create_child_instance(&self, change_note: String, change_type: VersionLevel) -> Self {
        Self {
            datetime: Zoned::now(),
            change_note,
            instance_type: InstanceType::Update,
            version: self.version.create_child_version(change_type),
        }
    }
    
    pub fn create_deletion_instance(&self, note: Option<String>) -> Self {
        Self {
            datetime: Zoned::now(),
            change_note: note.unwrap_or(String::from("Instance Deleted")),
            instance_type: InstanceType::Deletion,
            version: self.version.create_child_version(VersionLevel::Major),
        }
    }
    
    pub fn create_restored_instance(&self, note: Option<String>) -> Self {
        Self {
            datetime: Zoned::now(),
            change_note: note.unwrap_or(String::from("Instance restored")),
            instance_type: InstanceType::Restoration,
            version: self.version.create_child_version(VersionLevel::Major),
        }
    }
    
    pub fn get_version(&self) -> &Version {
        &self.version
    }
    
    pub fn get_datetime(&self) -> &Zoned {
        &self.datetime
    }
    
    pub fn get_change_note(&self) -> &str {
        &self.change_note
    }
    
    pub fn is_type_of(&self, instance_type: InstanceType) -> bool {
        self.instance_type == instance_type
    }
}

pub trait Instanced {
    fn get_instance(&self) -> &Instance;
}

pub struct InstanceList<T: Instanced> {
    instances: Vec<T>,
}

impl<T: Instanced> InstanceList<T> {
    pub fn new(mut values: Vec<T>) -> Self {
        values.sort_by(|a, b| a.get_instance().datetime.cmp(&b.get_instance().datetime));
        
        Self {
            instances: values,
        }
    }

    pub fn add(&mut self, new_instance: T) -> Result<(), InstanceError> {
        match self.latest() {
            Some(last_instance) => {
                if new_instance.get_instance().datetime < last_instance.get_instance().datetime {
                    return Err(InstanceError::DatetimeIncorrectlyOrdered);
                }
            }
            _ => (),
        }
        
        if self.is_deleted() && !new_instance.get_instance().is_type_of(InstanceType::Restoration) {
            return Err(InstanceError::CannotAddToDeletedInstanceList);
        }

        self.instances.push(new_instance);
        
        Ok(())
    }

    pub fn latest(&self) -> Option<&T> {
        self.instances.last()
    }
    
    pub fn earliest(&self) -> Option<&T> {
        self.instances.first()
    }
    
    pub fn len(&self) -> usize {
        self.instances.len()
    }
    
    pub fn is_deleted(&self) -> bool {
        match self.latest() {
            Some(instance) => instance.get_instance().is_type_of(InstanceType::Deletion),
            None => false,
        }
    }
}

#[derive(Debug)]
pub enum InstanceError {
    CannotAddToDeletedInstanceList,
    DatetimeIncorrectlyOrdered,
}

impl std::error::Error for InstanceError {}

impl Display for InstanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InstanceError::CannotAddToDeletedInstanceList => write!(f, "Cannot add to a deleted instance list"),
            InstanceError::DatetimeIncorrectlyOrdered => write!(f, "New instance datetime is before the latest instance datetime"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::VersionLevel;
    
    struct TestInstance {
        instance: Instance,
    }

    impl Clone for TestInstance {
        fn clone(&self) -> Self {
            Self {
                instance: Instance {
                    datetime: self.instance.datetime.clone(),
                    change_note: self.instance.change_note.clone(),
                    instance_type: self.instance.instance_type.clone(),
                    version: self.instance.version.clone(),
                }
            }
        }
    }

    impl Instanced for TestInstance {
        fn get_instance(&self) -> &Instance {
            &self.instance
        }
    }
    
    #[test]
    fn test_instance_list() {
        let instance1 = TestInstance {
            instance: Instance::create_initial_instance(VersionLevel::Minor),
        };
        
        let instance2 = TestInstance {
            instance: instance1.get_instance().create_child_instance(String::from("Test Change"), VersionLevel::Patch),
        };
        
        let instance3 = TestInstance {
            instance: instance2.get_instance().create_child_instance(String::from("Test Change 2"), VersionLevel::Patch),
        };
        
        let mut instance_list = InstanceList::new(vec![instance1, instance2]);
        
        assert_eq!(instance_list.len(), 2);
        assert_eq!(instance_list.latest().unwrap().get_instance().get_change_note(), "Test Change");
        
        instance_list.add(instance3.clone()).unwrap();
        
        assert_eq!(instance_list.len(), 3);
        assert_eq!(instance_list.latest().unwrap().get_instance().get_change_note(), "Test Change 2");
        
        let instance4 = TestInstance {
            instance: instance3.get_instance().create_deletion_instance(None),
        };
        
        instance_list.add(instance4.clone()).unwrap();
        
        assert_eq!(instance_list.len(), 4);
        assert_eq!(instance_list.latest().unwrap().get_instance().is_type_of(InstanceType::Deletion), true);
        
        let instance5 = TestInstance {
            instance: instance4.get_instance().create_restored_instance(None),
        };
        
        instance_list.add(instance5).unwrap();
        
        assert_eq!(instance_list.len(), 5);
        assert_eq!(instance_list.latest().unwrap().get_instance().is_type_of(InstanceType::Deletion), false);
    }
}