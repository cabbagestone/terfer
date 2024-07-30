use uuid::Uuid;
use crate::file_name::FileName;
use crate::instance::{Instance, Instanced, InstanceError, InstanceList};
use crate::tag::{Tag, TagError};
use crate::version::VersionLevel;

struct Item {
    id: String,
    instances: InstanceList<ItemInstance>,
    containing_folder: String,
    file_extension: String,
    file_type: FileType,
    file_title: Option<String>,
    tags: Vec<Tag>,
}

impl Item {
    pub fn new(containing_folder: String, file_extension: String, file_type: FileType) -> Result<Self, ItemError> {
        if containing_folder.ends_with('/') {
            return Err(ItemError::FilePath(String::from("Folder path cannot end with a slash")));
        }
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            instances: InstanceList::new(Vec::from([ItemInstance::new()])),
            containing_folder,
            file_extension,
            file_type,
            file_title: None,
            tags: Vec::new(),
        })
    }
    
    pub fn edit_title(&mut self, title: String) {
        self.file_title = Some(title);
    }

    pub fn edit(&mut self, note: String, version_level: VersionLevel) -> Result<(), ItemError> {
        let item_instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(ItemError::EditEmptyItem),
        };

        let new_instance = item_instance.get_instance().create_child_instance(note, version_level);
        self.instances.add(ItemInstance::with_instance(FileName::new(new_instance.get_version().clone()), new_instance))?;

        Ok(())
    }

    pub fn delete(&mut self, note: Option<String>) -> Result<(), ItemError> {
        let item_instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(ItemError::EditEmptyItem),
        };

        let new_instance = item_instance.get_instance().create_deletion_instance(note);
        self.instances.add(ItemInstance::with_instance(item_instance.file_name.clone(), new_instance))?;

        Ok(())
    }

    pub fn restore(&mut self, note: Option<String>) -> Result<(), ItemError> {
        let item_instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(ItemError::EditEmptyItem),
        };

        let new_instance = item_instance.get_instance().create_restoration_instance(note);
        self.instances.add(ItemInstance::with_instance(item_instance.file_name.clone(), new_instance))?;

        Ok(())
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
    
    pub fn remove_tag(&mut self, tag_id: &str) -> Result<(), ItemError> {
        let tag_index = self.tags.iter().position(|tag| tag.get_id().eq(tag_id));
        
        match tag_index {
            Some(index) => {
                self.tags.remove(index);
                Ok(())
            }
            None => Err(ItemError::TagNotFound),
        }
    }
    
    pub fn current_file_path(&self) -> Result<String, ItemError> {
        let instance = match self.instances.latest() {
            Some(instance) => instance,
            None => return Err(ItemError::RetrieveEmptyItem),
        };
        
        Ok(format!("{}/{}.{}", self.containing_folder, instance.file_name.to_string().unwrap(), self.file_extension))
    }
}

#[derive(Debug)]
pub enum ItemError {
    TagNotFound,
    EditEmptyItem,
    RetrieveEmptyItem,
    FilePath(String),
    Instance(InstanceError),
    Tag(TagError),
}

impl std::error::Error for ItemError {}

impl From<InstanceError> for ItemError {
    fn from(e: InstanceError) -> ItemError {
        ItemError::Instance(e)
    }
}

impl From<TagError> for ItemError {
    fn from(e: TagError) -> ItemError {
        ItemError::Tag(e)
    }
}

impl std::fmt::Display for ItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ItemError::Instance(e) => write!(f, "Item instance error: {}", e),
            ItemError::Tag(e) => write!(f, "Item tag error: {}", e),
            ItemError::TagNotFound => write!(f, "Tag not found"),
            ItemError::EditEmptyItem => write!(f, "Cannot edit an empty item"),
            ItemError::RetrieveEmptyItem => write!(f, "Cannot retrieve an empty item"),
            ItemError::FilePath(e) => write!(f, "Path error: {}", e),
        }
    }
}

struct ItemInstance {
    id: String,
    file_name: FileName,
    instance_meta: Instance,
}

impl ItemInstance {
    pub fn new() -> Self {
        let instance = Instance::create_initial_instance(VersionLevel::Minor);
        Self {
            id: Uuid::new_v4().to_string(),
            file_name: FileName::new(instance.get_version().clone()),
            instance_meta: Instance::create_initial_instance(VersionLevel::Minor),
        }
    }

    pub fn with_instance(file_name: FileName, instance: Instance) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            file_name,
            instance_meta: instance,
        }
    }
}

impl Instanced for ItemInstance {
    fn get_instance(&self) -> &Instance {
        &self.instance_meta
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use crate::version::Version;
    use super::*;
    
    #[test]
    fn test_item() -> Result<(), ItemError> {
        let folder_location = String::from("res/files/12154-15152-125");
        
        let mut item = Item::new(folder_location, String::from("jpeg"), FileType::Image)?;
        
        item.edit(String::from("Test Change"), VersionLevel::Minor).unwrap();
        item.delete(None).unwrap();
        assert!(item.instances.is_deleted());
        
        item.restore(None).unwrap();
        assert!(!item.instances.is_deleted());
        assert_eq!(item.instances.latest().unwrap().get_instance().get_version(), &Version::new(2, 0, 0));
        
        let tag = Tag::new(String::from("Test Tag"));
        let tag_id = tag.get_id().to_string();
        item.add_tag(tag);
        assert_eq!(item.tags.len(), 1);
        
        item.remove_tag(&tag_id).unwrap();
        assert_eq!(item.tags.len(), 0);
        
        Ok(())
    }
}