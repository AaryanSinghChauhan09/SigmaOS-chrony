// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-chrony: chrony component model.
//!
//! Real implementation: chrony_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChronyItem {
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
pub enum ChronyError {
    EmptyChronyItemName,
    DuplicateChronyItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ChronyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyChronyItemName => write!(f, "chrony_item name must not be empty"),
            Self::DuplicateChronyItem(id) => write!(f, "duplicate chrony_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ChronyError {}

pub struct Chrony {
    pub name: String,
    chrony_items: BTreeMap<String, ChronyItem>,
    events: BTreeMap<String, Event>,
}

impl Chrony {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            chrony_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_chrony_item(&mut self, item: ChronyItem) -> Result<(), ChronyError> {
        if item.name.trim().is_empty() {
            return Err(ChronyError::EmptyChronyItemName);
        }
        let id = item.id.clone();
        if self.chrony_items.contains_key(&id) {
            return Err(ChronyError::DuplicateChronyItem(id.clone()));
        }
        self.chrony_items.insert(id, item);
        Ok(())
    }

    pub fn chrony_item_count(&self) -> usize {
        self.chrony_items.len()
    }

    pub fn get_chrony_item(&self, id: &str) -> Option<&ChronyItem> {
        self.chrony_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ChronyError> {
        if event.subject.trim().is_empty() {
            return Err(ChronyError::EmptyEventSubject);
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

    fn sample_chrony_item(id: &str, name: &str) -> ChronyItem {
        ChronyItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Chrony::new("Chrony");
        assert_eq!(m.name, "Chrony");
        assert_eq!(m.chrony_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_chrony_item() {
        let mut m = Chrony::new("Chrony");
        m.add_chrony_item(sample_chrony_item("a1", "Alpha")).unwrap();
        assert_eq!(m.chrony_item_count(), 1);
        assert!(m.get_chrony_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Chrony::new("Chrony");
        assert!(m.add_chrony_item(sample_chrony_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Chrony::new("Chrony");
        m.add_chrony_item(sample_chrony_item("x", "One")).unwrap();
        assert!(m.add_chrony_item(sample_chrony_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Chrony::new("Chrony");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

/// Time synchronization service (NTP/timesyncd-style), extracted from
/// the SigmaOS-system monolith.
pub mod sigmatime;
