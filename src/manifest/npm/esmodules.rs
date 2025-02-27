use std::collections::HashMap;

use crate::manifest::npm::repository::Repository;

#[derive(Serialize)]
#[serde(untagged)]
pub enum SideEffects {
    Bool(bool),
    StringVec(Vec<String>),
}

#[derive(Serialize)]
pub struct ESModulesPackage {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collaborators: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    pub module: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(rename = "sideEffects")]
    pub side_effects: SideEffects,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
}
