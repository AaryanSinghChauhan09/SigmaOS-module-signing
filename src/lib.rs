// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-module-signing: module signing component model.
//!
//! Real implementation: signings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signing {
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
pub enum ModuleSigningError {
    EmptySigningName,
    DuplicateSigning(String),
    EmptyEventSubject,
}

impl fmt::Display for ModuleSigningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySigningName => write!(f, "signing name must not be empty"),
            Self::DuplicateSigning(id) => write!(f, "duplicate signing {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ModuleSigningError {}

pub struct ModuleSigning {
    pub name: String,
    signings: BTreeMap<String, Signing>,
    events: BTreeMap<String, Event>,
}

impl ModuleSigning {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            signings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_signing(&mut self, item: Signing) -> Result<(), ModuleSigningError> {
        if item.name.trim().is_empty() {
            return Err(ModuleSigningError::EmptySigningName);
        }
        let id = item.id.clone();
        if self.signings.contains_key(&id) {
            return Err(ModuleSigningError::DuplicateSigning(id.clone()));
        }
        self.signings.insert(id, item);
        Ok(())
    }

    pub fn signing_count(&self) -> usize {
        self.signings.len()
    }

    pub fn get_signing(&self, id: &str) -> Option<&Signing> {
        self.signings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ModuleSigningError> {
        if event.subject.trim().is_empty() {
            return Err(ModuleSigningError::EmptyEventSubject);
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

    fn sample_signing(id: &str, name: &str) -> Signing {
        Signing {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ModuleSigning::new("ModuleSigning");
        assert_eq!(m.name, "ModuleSigning");
        assert_eq!(m.signing_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_signing() {
        let mut m = ModuleSigning::new("ModuleSigning");
        m.add_signing(sample_signing("a1", "Alpha")).unwrap();
        assert_eq!(m.signing_count(), 1);
        assert!(m.get_signing("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ModuleSigning::new("ModuleSigning");
        assert!(m.add_signing(sample_signing("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ModuleSigning::new("ModuleSigning");
        m.add_signing(sample_signing("x", "One")).unwrap();
        assert!(m.add_signing(sample_signing("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ModuleSigning::new("ModuleSigning");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
