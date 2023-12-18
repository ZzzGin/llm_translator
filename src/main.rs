use crate::{
    models::translator_output::TranslatorOutput, translators::gpt::translates,
};
use arboard::Clipboard;
use std::{env, error::Error};
use utils::command_line::PrintCommand;
mod apis;
mod models;
mod translators;
mod utils;

const TEST_PROMPT_PATH: &str = "./prompts/eng_to_chn.txt";

#[tokio::main]
async fn main() {
    let original: String = env::args().nth(1).expect("Please provide content!");
    match translates(&original, TEST_PROMPT_PATH).await {
        Ok(translator_output) => handle_success(translator_output),
        Err(error) => handle_error(error),
    }
}

fn handle_success(translator_output: TranslatorOutput) {
    if translator_output.typo_map.is_empty() {
        let output: String = format!(
            "{}\n---\n{}",
            translator_output.original, translator_output.translated
        );
        PrintCommand::Success
            .print_message(&format!("Translation output:\n{}", output));
        copy_to_clipboard(&output);
        return;
    }

    let output: String = format!(
        "{}\n---\n{}",
        translator_output.fixed, translator_output.translated
    );
    let typo_output: String = format!(
        "Found potential typos in original content: [{}]",
        serde_json::to_string(&translator_output.typo_map).unwrap()
    );
    PrintCommand::Info.print_message(&typo_output);
    PrintCommand::Success
        .print_message(&format!("Fixed translation output:\n{}", output));
    copy_to_clipboard(&output);
}

fn handle_error(error: Box<dyn Error + Send>) {
    PrintCommand::Error.print_message(&error.to_string());
}

fn copy_to_clipboard(text: &String) {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.set_text(text) {
        Ok(_) => PrintCommand::Success
            .print_message(&"Copied in clipboard.".to_string()),
        Err(_) => PrintCommand::Error
            .print_message(&"Failed to copy into clipboard.".to_string()),
    }
}
