use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, PartialOrd)]
pub enum Shell {
    #[serde(alias = "fish")]
    Fish,
    #[serde(alias = "bash")]
    Bash,
    #[serde(alias = "zsh")]
    Zsh,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, PartialOrd)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    pub source_url: Option<String>,
    pub author: Option<String>,
    pub author_url: Option<String>,
    #[serde(default)]
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

    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Workflow {
            name: name.into(),
            command: command.into(),
            tags: vec![],
            description: None,
            arguments: vec![],
            source_url: None,
            author: None,
            author_url: None,
            shells: vec![],
        }
    }

    pub fn with_arguments(mut self, arguments: Vec<Argument>) -> Self {
        self.arguments = arguments;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, PartialOrd)]
pub struct Argument {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

impl Argument {
    pub fn new(name: impl Into<String>) -> Self {
        Argument {
            description: None,
            name: name.into(),
            default_value: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

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
