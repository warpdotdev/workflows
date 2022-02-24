#[macro_use]
extern crate derive_builder;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Shell {
    #[serde(rename(deserialize = "fish"))]
    Fish,
    #[serde(rename(deserialize = "bash"))]
    Bash,
    #[serde(rename(deserialize = "zsh"))]
    Zsh,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    #[builder(setter(into, strip_option), default)]
    pub source_url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub author: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub author_url: Option<String>,
    #[serde(default)]
    #[builder(default)]
    pub shells: Vec<Shell>,
}

impl Workflow {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn source_url(&self) -> &Option<String> {
        &self.source_url
    }

    pub fn author_name(&self) -> &Option<String> {
        &self.author
    }

    pub fn shells(&self) -> &Vec<Shell> {
        &self.shells
    }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Argument {
    pub name: String,
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub default_value: Option<String>,
}

impl Argument {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }
}
