use std::collections::HashMap;

use serenity::all::{Command, CommandId, CommandOption, CommandOptionType, Http};
use tokio::sync::OnceCell;

use crate::error::ServerError;

static COMMAND_MENTIONS: OnceCell<HashMap<String, String>> = OnceCell::const_new();

pub async fn initialize(http: impl AsRef<Http>) -> Result<(), ServerError> {
    COMMAND_MENTIONS
        .get_or_try_init(|| async {
            let commands = http.as_ref().get_global_commands().await?;
            let mut mentions = HashMap::new();

            for command in commands {
                let mut path = Vec::new();
                path.push(command.name.clone());
                insert_path(&mut mentions, &path, command.id);
                collect_options(&mut mentions, &mut path, &command);
            }

            Ok::<HashMap<String, String>, ServerError>(mentions)
        })
        .await?;

    Ok(())
}

pub fn get(key: &str) -> Option<&'static str> {
    COMMAND_MENTIONS
        .get()
        .and_then(|map| map.get(key))
        .map(|value| value.as_str())
}

fn collect_options(
    storage: &mut HashMap<String, String>,
    current_path: &mut Vec<String>,
    command: &Command,
) {
    for option in &command.options {
        match option.kind {
            CommandOptionType::SubCommand => {
                current_path.push(option.name.clone());
                insert_path(storage, current_path, command.id);
                current_path.pop();
            }
            CommandOptionType::SubCommandGroup => {
                current_path.push(option.name.clone());
                collect_group(storage, current_path, command.id, &option.options);
                current_path.pop();
            }
            _ => {}
        }
    }
}

fn collect_group(
    storage: &mut HashMap<String, String>,
    current_path: &mut Vec<String>,
    command_id: CommandId,
    options: &[CommandOption],
) {
    for option in options {
        if option.kind == CommandOptionType::SubCommand {
            current_path.push(option.name.clone());
            insert_path(storage, current_path, command_id);
            current_path.pop();
        }
    }
}

fn insert_path(storage: &mut HashMap<String, String>, path: &[String], command_id: CommandId) {
    let key = path.join("_");
    let label = path.join(" ");
    storage
        .entry(key)
        .or_insert_with(|| format!("</{}:{}>", label, command_id));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_before_initialize() {
        assert!(get("coin").is_none());
        assert!(get("refund_close").is_none());
    }

    #[test]
    fn insert_path_formats_a_mention() {
        let mut storage = HashMap::new();
        let id = CommandId::new(42);
        insert_path(
            &mut storage,
            &["refund".to_string(), "close".to_string()],
            id,
        );

        assert_eq!(
            storage.get("refund_close").map(String::as_str),
            Some("</refund close:42>")
        );
    }

    #[test]
    fn insert_path_does_not_overwrite_existing_entries() {
        let mut storage = HashMap::new();
        storage.insert("coin".to_string(), "existing".to_string());
        insert_path(&mut storage, &["coin".to_string()], CommandId::new(1));

        assert_eq!(storage.get("coin").map(String::as_str), Some("existing"));
    }

    #[test]
    fn collect_options_builds_subcommand_mentions() {
        let command: Command = serde_json::from_value(serde_json::json!({
            "id": "42",
            "type": 1,
            "application_id": "1",
            "name": "refund",
            "description": "Refund commands",
            "options": [
                { "type": 1, "name": "close", "description": "close a request" },
                { "type": 1, "name": "reopen", "description": "reopen a request" }
            ],
            "version": "1"
        }))
        .expect("command should deserialize");

        let mut storage = HashMap::new();
        let mut path = Vec::new();
        path.push(command.name.clone());
        insert_path(&mut storage, &path, command.id);
        collect_options(&mut storage, &mut path, &command);

        assert_eq!(
            storage.get("refund").map(String::as_str),
            Some("</refund:42>")
        );
        assert_eq!(
            storage.get("refund_close").map(String::as_str),
            Some("</refund close:42>")
        );
        assert_eq!(
            storage.get("refund_reopen").map(String::as_str),
            Some("</refund reopen:42>")
        );
    }

    #[test]
    fn collect_group_builds_nested_mentions() {
        let options: Vec<CommandOption> = serde_json::from_value(serde_json::json!([
            { "type": 1, "name": "nominate", "description": "nominate an option" },
            { "type": 1, "name": "revoke", "description": "revoke a nomination" }
        ]))
        .expect("options should deserialize");

        let mut storage = HashMap::new();
        let mut path = vec!["vote".to_string(), "group".to_string()];
        let id = CommandId::new(99);
        collect_group(&mut storage, &mut path, id, &options);

        assert_eq!(
            storage.get("vote_group_nominate").map(String::as_str),
            Some("</vote group nominate:99>")
        );
        assert_eq!(
            storage.get("vote_group_revoke").map(String::as_str),
            Some("</vote group revoke:99>")
        );
    }
}
