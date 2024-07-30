use jiff::fmt::strtime::format;
use jiff::Zoned;
use crate::version::Version;

const FILE_NAME_DATETIME_FORMAT: &'static str = "%Y-%m-%d-%H-%M-%S-%f%z";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileName {
    // TODO: Add suffix
    datetime: Zoned,
    version: Version,
}

impl FileName {
    pub fn from_string(file_name: &str) -> Result<Self, FileNameError> {
        let parts: Vec<&str> = file_name.split('_').collect();
        let datetime = Zoned::strptime(FILE_NAME_DATETIME_FORMAT, parts[0])?;
        let version = Version::from_string(parts[1]).unwrap();
        
        Ok(Self {
            datetime,
            version,
        })
    }
    
    pub fn new(version: Version) -> Self {
        Self {
            datetime: Zoned::now(),
            version,
        }
    }
    
    pub fn get_version(&self) -> &Version {
        &self.version
    }
    
    pub fn get_datetime(&self) -> &Zoned {
        &self.datetime
    }
    
    pub fn to_string(&self) -> Result<String, FileNameError> {
        let datetime = format(FILE_NAME_DATETIME_FORMAT, &self.datetime)?;
        Ok(format!("{}_{}", datetime, self.version.file_safe_string()))
    }
}

#[derive(Debug)]
pub enum FileNameError {
    FileUrlDateTime(jiff::Error),
}

impl From<jiff::Error> for FileNameError {
    fn from(e: jiff::Error) -> Self {
        FileNameError::FileUrlDateTime(e)
    }
}

impl std::error::Error for FileNameError {}

impl std::fmt::Display for FileNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileNameError::FileUrlDateTime(e) => write!(f, "File URL DateTime Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_name_from_string() {
        let file_name = FileName::from_string("2024-07-30-00-56-25-031870928-0600_1-2-3").unwrap();
        println!("{}", file_name.get_datetime().strftime("%F-%H-%M-%S").to_string());
        assert_eq!(file_name.get_datetime().strftime("%F-%H-%M-%S").to_string(), "2024-07-30-00-56-25");
        assert_eq!(file_name.get_version().to_string(), "1.2.3");
    }
    
    #[test]
    fn test_file_name_to_string() {
        let file_name = FileName::new(Version::new(1, 2, 3));
        assert_eq!(file_name.to_string().unwrap(), format!("{}_{}", file_name.get_datetime().strftime(FILE_NAME_DATETIME_FORMAT).to_string(), file_name.get_version().file_safe_string()));
    }
}