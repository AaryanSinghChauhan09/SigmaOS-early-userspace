// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-early-userspace: early userspace component model.
//!
//! Real implementation: userspaces, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Userspace {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EarlyUserspaceError {
    EmptyUserspaceName,
    DuplicateUserspace(String),
    EmptyEventSubject,
}

impl fmt::Display for EarlyUserspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUserspaceName => write!(f, "userspace name must not be empty"),
            Self::DuplicateUserspace(id) => write!(f, "duplicate userspace {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EarlyUserspaceError {}

pub struct EarlyUserspace {
    pub name: String,
    userspaces: BTreeMap<String, Userspace>,
    events: BTreeMap<String, Event>,
}

impl EarlyUserspace {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            userspaces: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_userspace(&mut self, item: Userspace) -> Result<(), EarlyUserspaceError> {
        if item.name.trim().is_empty() {
            return Err(EarlyUserspaceError::EmptyUserspaceName);
        }
        let id = item.id.clone();
        if self.userspaces.contains_key(&id) {
            return Err(EarlyUserspaceError::DuplicateUserspace(id.clone()));
        }
        self.userspaces.insert(id, item);
        Ok(())
    }

    pub fn userspace_count(&self) -> usize {
        self.userspaces.len()
    }

    pub fn get_userspace(&self, id: &str) -> Option<&Userspace> {
        self.userspaces.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EarlyUserspaceError> {
        if event.subject.trim().is_empty() {
            return Err(EarlyUserspaceError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_userspace(id: &str, name: &str) -> Userspace {
        Userspace {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EarlyUserspace::new("EarlyUserspace");
        assert_eq!(m.name, "EarlyUserspace");
        assert_eq!(m.userspace_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_userspace() {
        let mut m = EarlyUserspace::new("EarlyUserspace");
        m.add_userspace(sample_userspace("a1", "Alpha")).unwrap();
        assert_eq!(m.userspace_count(), 1);
        assert!(m.get_userspace("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EarlyUserspace::new("EarlyUserspace");
        assert!(m.add_userspace(sample_userspace("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EarlyUserspace::new("EarlyUserspace");
        m.add_userspace(sample_userspace("x", "One")).unwrap();
        assert!(m.add_userspace(sample_userspace("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EarlyUserspace::new("EarlyUserspace");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
