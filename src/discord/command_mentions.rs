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
                let mut path = vec![command.name.clone()];
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
