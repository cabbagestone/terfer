use std::fmt::{Debug, Display};
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

pub enum VersionChange {
    Major,
    Minor,
    Patch,
}

impl Version {
    pub fn from_string(version: &str) -> Result<Version, VersionError> {
        let parts: Vec<&str> = version.split('.').collect();

        if parts.len() != 3 {
            return Err(VersionError::InvalidVersionString(version.to_string()));
        }

        Ok(Version {
            major: parts[0].parse()?,
            minor: parts[1].parse()?,
            patch: parts[2].parse()?,
        })
    }
    pub fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn increment(&mut self, change: VersionChange) {
        match change {
            VersionChange::Major => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0;
            }
            VersionChange::Minor => {
                self.minor += 1;
                self.patch = 0;
            }
            VersionChange::Patch => self.patch += 1,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug)]
pub enum VersionError {
    InvalidVersionString(String),
}

impl From<ParseIntError> for VersionError {
    fn from(e: ParseIntError) -> VersionError {
        VersionError::InvalidVersionString(e.to_string())
    }
}

impl std::error::Error for VersionError {}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VersionError::InvalidVersionString(version) => write!(f, "Invalid version string: {}", version),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_from_string() {
        let version = Version::from_string("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_to_string() {
        let version = Version::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_increment() {
        let mut version = Version::new(1, 2, 3);
        version.increment(VersionChange::Major);
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);

        version.increment(VersionChange::Minor);
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 0);

        version.increment(VersionChange::Patch);
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 1);
    }

    #[test]
    fn test_version_from_string_error() {
        let version = Version::from_string("1.2");
        assert!(version.is_err());
    }
    
    #[test]
    fn test_version_error_display() {
        let version = Version::from_string("1.2");
        assert_eq!(version.unwrap_err().to_string(), "Invalid version string: 1.2");
    }
    
    #[test]
    fn test_version_error_from() {
        let error = VersionError::from("".parse::<u16>().unwrap_err());
        assert_eq!(error.to_string(), "Invalid version string: cannot parse integer from empty string");
    }
    
    #[test]
    fn test_equality() {
        let version1 = Version::new(1, 2, 3);
        let version2 = Version::new(1, 2, 3);
        assert_eq!(version1, version2);
    }
    
    #[test]
    fn test_inequality() {
        let version1 = Version::new(1, 2, 3);
        let version2 = Version::new(1, 2, 4);
        assert_ne!(version1, version2);
    }
    
    #[test]
    fn test_copy() {
        let version1 = Version::new(1, 2, 3);
        let mut version2 = version1;
        version2.increment(VersionChange::Major);
        assert_eq!(version1.major, 1);
        assert_eq!(version2.major, 2);
    }
    
    #[test]
    fn test_clone() {
        let version1 = Version::new(1, 2, 3);
        let mut version2 = version1.clone();
        version2.increment(VersionChange::Major);
        assert_eq!(version1.major, 1);
        assert_eq!(version2.major, 2);
    }
    
    #[test]
    fn test_debug() {
        let version = Version::new(1, 2, 3);
        assert_eq!(format!("{:?}", version), "Version { major: 1, minor: 2, patch: 3 }");
    }
}