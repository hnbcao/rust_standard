use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::assets;


#[derive(Clone, PartialEq)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
    commit: Option<String>,
    description: &'static str,
}

impl Default for Version {
    fn default() -> Self {
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<usize>().unwrap_or(0);
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<usize>().unwrap_or(0);
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<usize>().unwrap_or(0);
        let description = env!("CARGO_PKG_DESCRIPTION");
        let commit = assets::COMMIT.to_string().into();
        Version {
            major,
            minor,
            patch,
            commit,
            description,
        }
    }
}

impl Version {
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
            commit: None,
            description: "",
        }
    }
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn value(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn commit_id(&self) -> String {
        self.commit.clone().unwrap_or("unknown".to_string())
    }

    pub fn description(&self) -> &'static str {
        self.description
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.major
                .cmp(&other.major)
                .then_with(|| self.minor.cmp(&other.minor))
                .then_with(|| self.patch.cmp(&other.patch)),
        )
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        let mut parts: Vec<usize> = s.split('.').map(|part| part.parse::<usize>().unwrap_or(0)).collect();

        parts.resize(3, 0);

        Version::new(parts[0], parts[1], parts[2])
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.commit {
            Some(c) => write!(f, "v{}.{}.{}.{}", self.major, self.minor, self.patch, c),
            None => write!(f, "V{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}