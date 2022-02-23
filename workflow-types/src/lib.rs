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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    pub source: Option<String>,
    pub author: Option<String>,
    pub author_url: Option<String>,
    #[serde(default)]
    pub shells: Vec<Shell>,
}

impl Workflow {
    pub fn with_name_and_command(name: String, command: String) -> Self {
        Self {
            name,
            tags: vec![],
            command,
            description: None,
            arguments: vec![],
            source: None,
            author: None,
            author_url: None,
            shells: vec![],
        }
    }

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

    pub fn source(&self) -> &Option<String> {
        &self.source
    }

    pub fn author(&self) -> &Option<String> {
        &self.author
    }

    pub fn shells(&self) -> &Vec<Shell> {
        &self.shells
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Argument {
    pub name: String,
    pub identifier: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

impl Argument {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }
}
