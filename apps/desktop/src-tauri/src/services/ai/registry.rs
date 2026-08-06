use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommandDefinition {
    pub id: String,
    pub description: String,
    pub requires_input: bool,
    pub requires_context: bool,
}

#[derive(Debug, Default)]
pub struct CommandRegistry;

impl CommandRegistry {
    pub fn list(&self) -> Vec<CommandDefinition> {
        vec![
            CommandDefinition {
                id: "rewrite".to_string(),
                description: "Rewrite the selected text more clearly.".to_string(),
                requires_input: true,
                requires_context: false,
            },
            CommandDefinition {
                id: "reply".to_string(),
                description: "Draft a reply based on the selected text.".to_string(),
                requires_input: true,
                requires_context: true,
            },
            CommandDefinition {
                id: "translate".to_string(),
                description: "Translate the selected text.".to_string(),
                requires_input: true,
                requires_context: false,
            },
            CommandDefinition {
                id: "fix".to_string(),
                description: "Fix grammar and spelling in the selected text.".to_string(),
                requires_input: true,
                requires_context: false,
            },
            CommandDefinition {
                id: "summarize".to_string(),
                description: "Summarize the selected text.".to_string(),
                requires_input: true,
                requires_context: false,
            },
        ]
    }

    pub fn get(&self, id: &str) -> Option<CommandDefinition> {
        self.list().into_iter().find(|command| command.id == id)
    }
}
