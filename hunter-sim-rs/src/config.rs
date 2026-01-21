//! Configuration structures for loading build YAML files

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// The type of hunter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HunterType {
    Borge,
    Ozzy,
    Knox,
}

// Custom deserializer for case-insensitive matching
impl<'de> Deserialize<'de> for HunterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "borge" => Ok(HunterType::Borge),
            "ozzy" => Ok(HunterType::Ozzy),
            "knox" => Ok(HunterType::Knox),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["borge", "ozzy", "knox", "Borge", "Ozzy", "Knox"],
            )),
        }
    }
}

/// Metadata about the build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub hunter: HunterType,
    pub level: i32,
}

/// Full build configuration loaded from YAML/JSON
/// Supports both formats:
/// 1. { "meta": { "hunter": "Borge", "level": 69 }, ... }  (original YAML format)
/// 2. { "hunter": "Borge", "level": 69, ... }             (GUI JSON format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    // Support both nested meta and flat format
    #[serde(default)]
    pub meta: Option<Meta>,
    // Flat format fields (alternative to meta)
    #[serde(default)]
    pub hunter: Option<HunterType>,
    #[serde(default)]
    pub level: Option<i32>,
    
    pub stats: HashMap<String, i32>,
    pub talents: HashMap<String, i32>,
    pub attributes: HashMap<String, i32>,
    #[serde(default)]
    pub inscryptions: HashMap<String, i32>,
    #[serde(default)]
    pub mods: HashMap<String, bool>,
    #[serde(default)]
    pub relics: HashMap<String, i32>,
    #[serde(default)]
    pub gems: HashMap<String, i32>,
    #[serde(default)]
    pub gadgets: HashMap<String, i32>,
    #[serde(default)]
    pub bonuses: HashMap<String, serde_json::Value>,
}

impl BuildConfig {
    /// Get the hunter type (from meta or flat format)
    pub fn get_hunter_type(&self) -> HunterType {
        if let Some(ref meta) = self.meta {
            meta.hunter
        } else {
            self.hunter.unwrap_or(HunterType::Borge)
        }
    }
    
    /// Get the level (from meta or flat format)
    pub fn get_level(&self) -> i32 {
        if let Some(ref meta) = self.meta {
            meta.level
        } else {
            self.level.unwrap_or(1)
        }
    }
    
    /// Load a build configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        let path_str = path.as_ref().to_string_lossy().to_lowercase();
        
        // Check if it's JSON or YAML
        if path_str.ends_with(".json") {
            let config: BuildConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config: BuildConfig = serde_yaml::from_str(&content)?;
            Ok(config)
        }
    }
    
    /// Load from JSON string (for Python interop)
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: BuildConfig = serde_json::from_str(json)?;
        Ok(config)
    }
    
    /// Get a stat value with default
    pub fn get_stat(&self, key: &str) -> i32 {
        *self.stats.get(key).unwrap_or(&0)
    }
    
    /// Get a talent value with default
    pub fn get_talent(&self, key: &str) -> i32 {
        *self.talents.get(key).unwrap_or(&0)
    }
    
    /// Get an attribute value with default
    pub fn get_attr(&self, key: &str) -> i32 {
        *self.attributes.get(key).unwrap_or(&0)
    }
    
    /// Get an inscryption value with default
    pub fn get_inscr(&self, key: &str) -> i32 {
        *self.inscryptions.get(key).unwrap_or(&0)
    }
    
    /// Get a relic value with default
    pub fn get_relic(&self, key: &str) -> i32 {
        *self.relics.get(key).unwrap_or(&0)
    }
    
    /// Get a gem value with default
    pub fn get_gem(&self, key: &str) -> i32 {
        *self.gems.get(key).unwrap_or(&0)
    }
}
