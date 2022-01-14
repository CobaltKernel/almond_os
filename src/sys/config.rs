/// Basic Configuration Data
/// Stored In ASCII <KEY>=<VALUE> Pairing
/// Examples:
/// PROMPT_MSG=Shell>> 
/// GRAPHICS=TEXT


use alloc::{
    string::{String, ToString},
    vec::Vec,
    collections::BTreeMap
};

use crate::{slog};

/// Holds & Maintains System Configuration Info
#[derive(Debug)]
pub struct SystemConfig {
    map: BTreeMap<String, String>,
}

impl SystemConfig {
    /// Initializes A New System Config from a str
    pub fn from_str(s: &str) -> SystemConfig {
        let lines = s.lines();
        let mut map = BTreeMap::new();
        for line in lines {
            let sections: Vec<String> = line.split("=").map(|s| {s.to_string()}).collect();
            if sections.len() > 1 {
                let key = sections[0].to_string();
                let value = sections[1].to_string();
                map.insert(key, value);
            } else {
                slog!("Malformatted Line '{}'\n", line);
            }
        }

        SystemConfig {
            map
        }
    }

    /// Returns The Value At Key, Returns None If Key Is Missing or The Value Cannot be parsed
    /// into a Base-10 Number.
    pub fn get_usize(&self, key: &str) -> Option<usize> {
        if self.map.contains_key(key) {
            let result = usize::from_str_radix(self.map.get(key).unwrap(), 10);
            match result {
                Err(_) => return None,
                Ok(x) => return Some(x),
            }
        } else {
            return None;
        }
    }
    
    /// Returns The Value, Or None If Missing.
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}